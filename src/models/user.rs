use std::error::Error;
use crate::models::user_permission::Permission;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Pool, Sqlite};
use std::sync::Arc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;
use crate::util::jwt::claims::Claims;

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

    /// writes user to db
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


    /// gets user by token
    pub async fn from_token(token: String, jwt_secret: String, conn: &Arc<Pool<Sqlite>>) -> Result<Option<User>, Box<dyn Error>> {
        // decode token
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        // validate claims
        let claims = token_data.claims;
        if !claims.valid_dates() {
            return Ok(None)
        }

        // get user
        let user = User::from_claims(claims, conn).await?;

        // check for tokenversion
        if &claims.tokenversion != &user.tokenversion {
            return Ok(None)
        }

        Ok(Some(user))
    }

    /// gets user from db with uuid form claims
    /// DOES NOT CHECK FOR VALIDATION
    pub async fn from_claims(claims: Claims, conn: &Arc<Pool<Sqlite>>) -> Result<User, sqlx::Error> {
        let query = r"SELECT * FROM users WHERE uuid = ?";
        let user = sqlx::query_as::<_, User>(query)
            .bind(claims.sub)
            .fetch_one(conn)
            .await?;
        Ok(user)
    }
}