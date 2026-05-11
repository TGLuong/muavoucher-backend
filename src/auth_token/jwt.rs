use std::sync::Arc;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::auth_token::{AuthTokenError, AuthTokenTrait, Claims};

#[derive(Debug, Clone)]
pub struct JwtAuthToken {
    secret: Arc<String>,
}

impl JwtAuthToken {
    pub fn new(secret: String) -> Self {
        Self { secret: Arc::new(secret) }
    }
}

impl AuthTokenTrait for JwtAuthToken {
    fn generate(&self, claims: Claims) -> Result<String, AuthTokenError> {
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_bytes()))?;
        Ok(token)
    }

    fn validate(&self, token: &str) -> Result<Claims, AuthTokenError> {
        match decode::<Claims>(token, &DecodingKey::from_secret(self.secret.as_bytes()), &Validation::default()) {
            Ok(data) => Ok(data.claims),
            Err(err) => Err(AuthTokenError::Validation(err.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{Duration, Utc};

    use crate::auth_token::{AuthTokenTrait, Claims, jwt::JwtAuthToken};

    #[test_log::test]
    fn test_jwt() {
        let auth_token = JwtAuthToken::new("secret".into());
        let exp = Utc::now().checked_add_signed(Duration::days(10)).expect("valid timestamp").timestamp() as usize;
        let token = auth_token
            .generate(Claims {
                user_id: "id".into(),
                user_role: "user".into(),
                exp,
            })
            .expect("should ok");
        log::info!("token: {token}");
        let data = auth_token.validate(&token).expect("should ok");
        log::info!("data: {data:?}");
        assert_eq!(
            data,
            Claims {
                user_id: "id".into(),
                user_role: "user".into(),
                exp
            }
        );
    }
}
