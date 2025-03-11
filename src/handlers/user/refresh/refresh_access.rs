use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::PrivateCookieJar;
use uuid::Uuid;
use crate::models::appstate::AppstateWrapper;
use crate::models::auth_user::AuthUser;
use crate::util::jwt::access_token::AccessToken;
use crate::util::jwt::claims::Claims;


/// handler function to refresh access tokens
/// previous access must still be valid
#[axum_macros::debug_handler]
pub async fn regenerate_access_token(
    State(appstate_wrapper): State<AppstateWrapper>,
    auth_user: Extension<AuthUser>,
    jar: PrivateCookieJar,
) -> Result<(StatusCode, PrivateCookieJar), StatusCode> {
    let appstate = appstate_wrapper.0;
    let user = auth_user.0.0;

    let claims = Claims::new(user.uuid.into_uuid(), user.tokenversion, 20);
    let token = match AccessToken::from_claims(claims, &appstate.jwt_secret) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };
    let mut cookie = Cookie::new("access_token", token.token);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_http_only(true);
    let jar = jar.add(cookie);
    Ok((StatusCode::OK, jar))
}