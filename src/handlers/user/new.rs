use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::PrivateCookieJar;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use crate::models::appstate::{AppstateWrapper};
use crate::models::user::User;
use crate::util::hashing::hash_password;
use crate::util::validation::{valid_password, valid_username};

#[derive(Serialize, Deserialize)]
pub struct Body {
    username: String,
    password: String,
    email: String,
}

/// Handler for creating new users
#[axum_macros::debug_handler]
pub async fn create_new_user(
    State(appstate_wrapper): State<AppstateWrapper>,
    mut jar: PrivateCookieJar,
    Json(body): Json<Body>
) -> Result<(StatusCode, PrivateCookieJar), (StatusCode, &'static str)> {
    let appstate = appstate_wrapper.0;

    // validate password and username
    if !valid_username(&body.username) {
        return Err((StatusCode::BAD_REQUEST, "Bad username (do specific checks on frontend)"))
    }
    if !valid_password(&body.password)  {
        return Err((StatusCode::BAD_REQUEST, "Bad password (do specific checks on frontend)"))
    }
    // ! TODO email validation

    // hash password and create user model
    let hashed_password = match hash_password(&body.password).await {
        Ok(o) => o,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password")),
    };

    // create user
    let user = User::new(body.username, hashed_password, body.email);

    // add user to db
    let query = user.write_to_db(&appstate.db);
    match query.await {
        Ok(_) => {},
        Err(Error::Database(db_err)) => {
            return if db_err.message().contains("email") {
                Err((StatusCode::BAD_REQUEST, "Email is already in use"))
            } else if db_err.message().contains("username") {
                Err((StatusCode::BAD_REQUEST, "Username is already taken"))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to write to db"))
            }
            // technically the uuid could be the same here, and we would have an unhandled exception but when will that happen

        }
        _ => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to write to db"))
    }

    // set cookies
    let access_token = match user.generate_access_token(&appstate.jwt_secret) {
        Some(access_token) => access_token,
        None => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate access token please log in manually"))
    };
    let refresh_token = match user.generate_refresh_token(&appstate.jwt_secret) {
        Some(r_token) => r_token,
        None => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate refresh token please log in manually"))
    };
    println!("access: {}\nrefresh: {}", &access_token.token, &refresh_token.token);

    let mut r_cookie = Cookie::new("refresh_token", refresh_token.token);
    let mut a_cookie = Cookie::new("access_token", access_token.token);
    r_cookie.set_same_site(SameSite::Strict);
    r_cookie.set_http_only(true);
    a_cookie.set_same_site(SameSite::Strict);
    a_cookie.set_http_only(true);

    jar = jar.add(r_cookie).add(a_cookie);

    Ok((StatusCode::CREATED, jar))
}