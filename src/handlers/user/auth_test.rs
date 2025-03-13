use axum::{Extension, Json};
use axum::extract::State;
use axum_extra::extract::PrivateCookieJar;
use crate::models::appstate::AppstateWrapper;
use crate::models::auth_user::AuthUser;
use crate::models::user::User;


#[axum_macros::debug_handler]
pub async fn auth_test(
    State(wrapped_appstate): State<AppstateWrapper>,
    jar: PrivateCookieJar,
    user: Extension<AuthUser>,
) -> Json<User> {
    Json(user.0.0)
}