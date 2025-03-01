use std::error::Error;
use crate::models::user_permission::Permission;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use std::sync::Arc;
use uuid::Uuid;
use crate::util::jwt::access_token::AccessToken;
use crate::util::jwt::claims::Claims;
use crate::util::jwt::refresh_token::RefreshToken;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub(crate) uuid: uuid::fmt::Hyphenated,
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
            uuid: Uuid::new_v4().hyphenated(),
            username,
            password,
            email,
            permission: Permission::USER,
            tokenversion: 0,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }

    /// gets user by token
    pub async fn from_access_token(token: AccessToken, conn: &Arc<Pool<Sqlite>>) -> Result<Option<User>, Box<dyn Error>> {
        // validate claims
        let claims = token.claims;
        if !claims.valid_dates() {
            return Ok(None)
        }

        // get user
        let user = User::from_claims(claims.clone(), conn).await?;

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
            .bind(claims.sub.to_string())
            .fetch_one(conn.as_ref())
            .await?;
        Ok(user)
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
            .bind(self.tokenversion.clone() as u32) // we have to parse as u32 here as u64 doesn't meet trait requirements
            .bind(self.timestamp.clone() as u32).execute(conn.as_ref()).await?;

        Ok(())
    }

    /// generates access token (exp in 1y) for user
    pub fn generate_access_token(&self, jwt_secret: &String) -> Option<AccessToken> {
        let claims = Claims::from_user(&self, 60*24*365);
        match AccessToken::from_claims(claims, jwt_secret) {
            Ok(token) => Some(token),
            _ => None,
        }
    }

    /// generates refresh token (exp 20 minutes) for user
    pub fn generate_refresh_token(&self, jwt_secret: &String) -> Option<RefreshToken> {
        let claims = Claims::from_user(&self, 20);
        match RefreshToken::from_claims(claims, jwt_secret) {
            Ok(token) => Some(token),
            _ => None
        }
    }
}