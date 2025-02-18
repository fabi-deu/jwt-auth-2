use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::PrivateCookieJar;
use serde::{Deserialize, Serialize};
use crate::models::appstate::{AppstateWrapper};
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
async fn create_new_user(
    State(appstate_wrapper): State<AppstateWrapper>,
    jar: PrivateCookieJar,
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

    // hash password and create user model
    let hashed_password = match hash_password(&body.password).await {
        Ok(o) => o,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password")),
    };



    Ok((StatusCode::CREATED, jar))
}