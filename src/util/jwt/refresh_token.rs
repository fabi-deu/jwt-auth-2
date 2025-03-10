use crate::util::jwt::claims::Claims;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::util::jwt::access_token::AccessToken;

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

    /// generates new token from old
    pub fn refresh_token(self, jwt_secret: &String) -> jsonwebtoken::errors::Result<Self> {
        let old_claims = &self.claims;
        let new_claims = Claims::new(old_claims.sub, old_claims.tokenversion, 20);
        RefreshToken::from_claims(new_claims, jwt_secret)
    }
}