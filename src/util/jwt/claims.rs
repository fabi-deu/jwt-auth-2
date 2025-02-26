use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: Uuid,
    pub(crate) tokenversion: u64,
    pub(crate) iat: u64,
    pub(crate) exp: u64,
}


impl Claims {

    /// * `exp` - Describes in how many minutes the token will expire
    pub fn new(sub: Uuid, tokenversion: u64, exp: u64) -> Self {
        Self {
            sub,
            tokenversion,
            iat: Utc::now().timestamp() as u64,
            exp: Utc::now().timestamp() as u64 + exp*60,
        }
    }
    pub fn valid_dates(&self) -> bool {
        if self.exp < Utc::now().timestamp() as u64 {
            return false
        }
        if self.iat > Utc::now().timestamp() as u64 {
            return false
        }

        true
    }
}