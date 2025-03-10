use crate::models::appstate::Appstate;
use crate::models::user::User;
use axum::http::StatusCode;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::PrivateCookieJar;


/// generates both access and refresh token for user and adds it to the cookie jar, which is returned
pub fn generate_cookies(user: &User, mut jar: PrivateCookieJar, appstate: &Appstate) -> Result<PrivateCookieJar, (StatusCode, &'static str)> {
    let access_token = match user.generate_access_token(&appstate.jwt_secret) {
        Some(access_token) => access_token,
        None => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate access token please log in manually"))
    };
    let refresh_token = match user.generate_refresh_token(&appstate.jwt_secret) {
        Some(r_token) => r_token,
        None => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate refresh token please log in manually"))
    };

    let mut r_cookie = Cookie::new("refresh_token", refresh_token.token);
    let mut a_cookie = Cookie::new("access_token", access_token.token);
    r_cookie.set_same_site(SameSite::Strict);
    r_cookie.set_http_only(true);
    a_cookie.set_same_site(SameSite::Strict);
    a_cookie.set_http_only(true);

    jar = jar.add(r_cookie).add(a_cookie);

    Ok(jar)
}
