use axum::Extension;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::PrivateCookieJar;
use crate::models::appstate::AppstateWrapper;
use crate::models::auth_user::AuthUser;
use crate::models::user::User;
use crate::util::jwt::access_token::AccessToken;

pub async fn auth_middleware(
    Extension(appstate_wrapper): Extension<AppstateWrapper>,
    mut req: Request,
    next: Next
) -> Result<Response, StatusCode> {
    let appstate = appstate_wrapper.0;
    let headers = req.headers();

    // get cookies
    let jar = PrivateCookieJar::from_headers(headers, appstate.cookie_secret.clone());
    let cookie = match jar.get("access_token") {
        Some(c) => c,
        _ => return Err(StatusCode::BAD_REQUEST)
    };

    // get access token from cookies
    let token = match AccessToken::from_literal(cookie.value().to_string(), &appstate.jwt_secret) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // get user from access token
    let user = match User::from_access_token(token, &appstate.db).await {
        Ok(some_user) => {
            match some_user {
                Some(user) => user,
                _ => return Err(StatusCode::UNAUTHORIZED)
            }
        }
        Err(e) => {
            println!("{}", e.to_string());
            return Err(StatusCode::IM_A_TEAPOT)
        }
    };


    // pass wrapped user to next
    req.extensions_mut().insert(AuthUser(user));
    let response = next.run(req).await;
    Ok(response)
}