use axum::Extension;
use axum::extract::{State};
use axum::http::StatusCode;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::PrivateCookieJar;
use uuid::Uuid;
use crate::models::appstate::AppstateWrapper;
use crate::models::auth_user::AuthUser;
use crate::util::jwt::refresh_token::{RefreshToken};
use crate::util::jwt::claims::Claims;


/// handler function to refresh the refresh tokens
/// previous access token must still be valid
#[axum_macros::debug_handler]
pub async fn regenerate_refresh_token(
    State(appstate_wrapper): State<AppstateWrapper>,
    auth_user: Extension<AuthUser>,
    jar: PrivateCookieJar,
) -> Result<(StatusCode, PrivateCookieJar), StatusCode> {
    let appstate = appstate_wrapper.0;
    let user = auth_user.0.0;

    // new token
    let claims = Claims::new(user.uuid.into_uuid(), user.tokenversion, 20);
    let token = match RefreshToken::from_claims(claims, &appstate.jwt_secret) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    // set up cookie
    let mut cookie = Cookie::new("refresh_token", token.token);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_http_only(true);
    let jar = jar.add(cookie);
    Ok((StatusCode::OK, jar))



}


fn generate_token_from_zero(jar: PrivateCookieJar, sub: Uuid, tokenversion: u64, jwt_secret: &String) -> Result<(StatusCode, PrivateCookieJar), StatusCode> {
}