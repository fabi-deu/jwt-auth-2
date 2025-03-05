use crate::models::appstate::AppstateWrapper;
use crate::models::user::User;
use crate::util::cookies::generate_cookies;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::PrivateCookieJar;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Body {
    username: String,
    password: String,
}

/// login handler
#[axum_macros::debug_handler]
pub async fn login(
    State(appstate_wrapper): State<AppstateWrapper>,
    jar: PrivateCookieJar,
    Json(body): Json<Body>
) -> Result<(StatusCode, PrivateCookieJar), (StatusCode, &'static str)> {
    let appstate = appstate_wrapper.0;

    // fetch user from db
    let user: User = match User::from_username(body.username, &appstate.db).await {
        Ok(user) => user,
        // technically this could also be a db error, but realistically it's the users false input
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Failed to fetch user from db"))
    };


    // compare passwords
    match user.verify_password(body.password) {
        Ok(true) => {},
        Ok(false) => return Err((StatusCode::BAD_REQUEST, "Wrong password")),
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to verify password")),
    }

    // set up cookies
    let jar = generate_cookies(&user, jar, &appstate)?;

    Ok((StatusCode::OK, jar))
}