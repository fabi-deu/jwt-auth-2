use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::PrivateCookieJar;
use uuid::Uuid;
use crate::models::appstate::AppstateWrapper;
use crate::models::auth_user::AuthUser;
use crate::util::jwt::refresh_token::{RefreshToken};
use crate::util::jwt::claims::Claims;


/// handler function to refresh refresh tokens
#[axum_macros::debug_handler]
pub async fn regenerate_refresh_token(
    State(appstate_wrapper): State<AppstateWrapper>,
    auth_user: Extension<AuthUser>,
    jar: PrivateCookieJar,
) -> Result<(StatusCode, PrivateCookieJar), StatusCode> {
    let appstate = appstate_wrapper.0;
    let user = auth_user.0.0;

    // get cookie
    let prev_token = match jar.get("refresh_token") {
        Some(cookie) => cookie,
        None => {
            return generate_token_from_zero(jar, user.uuid.into_uuid(), user.tokenversion, &appstate.jwt_secret)
        }
    };

    // turn cookie into token
    let old_refresh_token = match RefreshToken::from_literal(prev_token.value().to_string(), &appstate.jwt_secret) {
        Ok(token) => token,
        Err(_) => {
            return generate_token_from_zero(jar, user.uuid.into_uuid(), user.tokenversion, &appstate.jwt_secret)
        }
    };

    // refresh old token
    let new_refresh_token = match old_refresh_token.refresh_token(&appstate.jwt_secret) {
        Ok(token) => token,
        Err(_) => {
            // try again
            return generate_token_from_zero(jar, user.uuid.into_uuid(), user.tokenversion, &appstate.jwt_secret)
        }
    };

    // set cookies
    let mut cookie = Cookie::new("refresh_token", new_refresh_token.token);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_http_only(true);
    let jar = jar.add(cookie);

    Ok((StatusCode::OK, jar))
}


fn generate_token_from_zero(jar: PrivateCookieJar, sub: Uuid, tokenversion: u64, jwt_secret: &String) -> Result<(StatusCode, PrivateCookieJar), StatusCode> {
    let claims = Claims::new(sub, tokenversion, 20);
    let token = match RefreshToken::from_claims(claims, jwt_secret) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };
    let mut cookie = Cookie::new("refresh_token", token.token);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_http_only(true);
    let jar = jar.add(cookie);
    Ok((StatusCode::OK, jar))
}