use crate::models::user_permission::Permission;
use argon2::PasswordHash;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub(crate) uuid: Uuid,
    pub(crate) username: String,
    password: PasswordHash<'static>,
    pub(crate) email: String,

    pub(crate) permission: Permission,
    pub(crate) tokenversion: u64,
    pub(crate) timestamp: u64,
}


impl User {
    pub fn new(username: String, password: PasswordHash, email: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username,
            password,
            email,
            permission: Permission::USER,
            tokenversion: 0,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }
}