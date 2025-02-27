use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::util::jwt::claims::Claims;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub(crate) claims: Claims,
    pub(crate) token: String,
}


impl AccessToken {
    /// generates access-token from claims with default header and given secret
    /// DOES NOT CHECK FOR VALIDATION
    /// exp should be a small
    pub fn from_claims(claims: Claims, jwt_secret: String) -> jsonwebtoken::errors::Result<Self> {
        // generate token with default headers
        let token =
            encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))?;
        Ok(Self {
            claims,
            token,
        })
    }

    pub fn refresh_token() -> () {
        todo!()
    }
}