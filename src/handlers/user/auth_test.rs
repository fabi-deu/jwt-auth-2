use axum::Json;
use crate::models::auth_user::AuthUser;
use crate::models::user::User;

pub async fn auth_test(
    AuthUser(auth_user): AuthUser,
) -> Json<User> {
    Json(auth_user)
}