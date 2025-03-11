use std::error::Error;
use std::future::Future;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::PrivateCookieJar;
use crate::models::appstate::AppstateWrapper;
use crate::models::user::User;
use crate::util::jwt::access_token::AccessToken;
use crate::util::jwt::claims::Claims;
use crate::util::jwt::refresh_token::RefreshToken;

pub async fn new_access_token(
    State(appstate_wrapper): State<AppstateWrapper>,
    jar: PrivateCookieJar,
) -> Result<(StatusCode, PrivateCookieJar), StatusCode> {
    let appstate = appstate_wrapper.0;

    // get old refresh token
    let refresh_token_cookie = match jar.get("refresh_token") {
        Some(c) => c,
        None => return Err(StatusCode::BAD_REQUEST)
    };
    let refresh_token = match RefreshToken::from_literal(refresh_token_cookie.value().to_string(), &appstate.jwt_secret) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };
    drop(refresh_token_cookie);

    // generate new access token
    let sub = refresh_token.claims.sub;
    let user = match User::from_uuid(sub.clone(), &appstate.db).await {
        Ok(user) => user,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    // check for token version
    if user.tokenversion != refresh_token.claims.tokenversion {
        return Err(StatusCode::UNAUTHORIZED)
    }
    if !refresh_token.claims.valid_dates() {
        return Err(StatusCode::UNAUTHORIZED)
    }

    let claims = Claims::new(sub, user.tokenversion, 20);
    let token = AccessToken::


    Ok((StatusCode::OK, jar))
}