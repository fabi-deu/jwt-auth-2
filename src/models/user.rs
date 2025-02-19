use std::sync::Arc;
use crate::models::user_permission::Permission;
use argon2::PasswordHash;
use serde::{Deserialize, Serialize};
use serde::de::Error;
use sqlx::{Pool, Sqlite};
use sqlx::pool::PoolConnection;
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


    pub async fn write_to_db(&self, conn: PoolConnection<Sqlite>) -> Result<(), Box<dyn Error>> {
        let query =
            r"INSERT INTO users (uuid, username, email, password, permission, tokenversion, timestamp) VALUES ($1, $2, $3, $4, $5, $6, $7)";

        let _ = sqlx::query(query)
            .bind(&self.uuid.to_string())
            .bind(&self.username)
            .bind(&self.email)
            .bind(&self.password.to_string())
            .bind(&self.permission.to_string())
            .bind(&self.tokenversion)
            .bind(&self.timestamp).execute(conn.as_ref()).await?;

        Ok(())
    }
}