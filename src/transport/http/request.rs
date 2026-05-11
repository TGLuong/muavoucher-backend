use std::str::FromStr;

use crate::{
    auth_token::AuthTokenTrait,
    otp_notifier::OtpNotifierTrait,
    storage::{entities::user::UserRole, kv_store::KVStoreTrait},
    transport::http::context::HttpContext,
};
use axum::{
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
};
use uuid::Uuid;

#[derive(Debug)]
pub struct OtpHeader(pub String);

impl<OTP, KV, AU> FromRequestParts<HttpContext<OTP, KV, AU>> for OtpHeader
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _state: &HttpContext<OTP, KV, AU>) -> Result<Self, Self::Rejection> {
        let otp = parts
            .headers
            .get("user_otp")
            .ok_or("header missing user_otp".to_string())?
            .to_str()
            .map_err(|e| e.to_string())?;
        Ok(Self(otp.to_string()))
    }
}

#[derive(Debug)]
pub struct AuthUserId {
    pub id: Uuid,
    pub role: UserRole,
}

impl<OTP, KV, AU> FromRequestParts<HttpContext<OTP, KV, AU>> for AuthUserId
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &HttpContext<OTP, KV, AU>) -> Result<Self, Self::Rejection> {
        let Path(id) = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
        let id = Uuid::from_str(&id).map_err(|e| (StatusCode::BAD_REQUEST, format!("User id invalid: {e:?}")))?;
        let token = parts
            .headers
            .get("user_token")
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header".into()))?;
        let claims = state
            .logic
            .user_authen(token)
            .await
            .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;
        (id.to_string() == claims.user_id)
            .then_some(())
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid Authorization Token".into()))?;
        Ok(AuthUserId {
            id,
            role: claims.user_role.into(),
        })
    }
}
