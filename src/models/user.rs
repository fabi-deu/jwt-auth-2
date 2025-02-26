use crate::models::user_permission::Permission;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Pool, Sqlite};
use std::sync::Arc;
use uuid::Uuid;


#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub(crate) uuid: Uuid,
    pub(crate) username: String,
    password: String,
    pub(crate) email: String,

    pub(crate) permission: Permission,
    pub(crate) tokenversion: u64,
    pub(crate) timestamp: u64,
}


impl User {
    pub fn new(username: String, password: String, email: String) -> Self {
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


    pub async fn write_to_db(&self, conn: &Arc<Pool<Sqlite>>) -> Result<(), sqlx::Error> {
        let query =
            r"INSERT INTO users (uuid, username, email, password, permission, tokenversion, timestamp) VALUES (?, ?, ?, ?, ?, ?, ?)";

        let _ = sqlx::query(query)
            .bind(&self.uuid.to_string())
            .bind(&self.username)
            .bind(&self.email)
            .bind(&self.password)
            .bind(&self.permission.to_string())
            .bind(self.tokenversion.clone() as u32)
            .bind(self.timestamp.clone() as u32).execute(conn.as_ref()).await?;

        Ok(())
    }
}