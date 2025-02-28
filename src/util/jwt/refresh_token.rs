use crate::util::jwt::claims::Claims;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    pub(crate) claims: Claims,
    pub(crate) token: String,
}


impl RefreshToken {
    /// generates refresh-token from claims with default header and given secret
    /// DOES NOT CHECK FOR VALIDATION
    /// exp should be long
    pub fn from_claims(claims: Claims, jwt_secret: &String) -> jsonwebtoken::errors::Result<Self> {
        // generate token with default headers
        let token =
            encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))?;
        Ok(Self {
            claims,
            token,
        })
    }
    /// this function returns the literal token
    pub fn to_string(&self) -> String {
        self.token.clone()
    }

}