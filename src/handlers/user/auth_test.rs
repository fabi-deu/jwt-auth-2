use axum::{Extension, Json};
use crate::models::auth_user::AuthUser;
use crate::models::user::User;


#[axum_macros::debug_handler]
pub async fn auth_test(
    user: Extension<AuthUser>
) -> Json<User> {
    Json(user.0.0)
}