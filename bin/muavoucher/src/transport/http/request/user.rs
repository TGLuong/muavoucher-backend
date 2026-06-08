use std::str::FromStr;

use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    auth_token::AuthTokenTrait,
    otp_notifier::OtpNotifierTrait,
    storage::{entities::user::UserRole, kv_store::KVStoreTrait},
    transport::http::context::HttpContext,
    webhook_validator::WebhookValidator,
};

#[derive(Debug)]
pub struct AdminUser;

impl<OTP, KV, AU, WU> FromRequestParts<HttpContext<OTP, KV, AU, WU>> for AdminUser
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &HttpContext<OTP, KV, AU, WU>) -> Result<Self, Self::Rejection> {
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
        let role: UserRole = claims.user_role.clone().into();
        (role == UserRole::Admin)
            .then_some(())
            .ok_or((StatusCode::UNAUTHORIZED, format!("claims {:?} is not admin", claims)))?;
        Ok(AdminUser)
    }
}

#[derive(Debug)]
pub struct AuthUserId {
    pub id: Uuid,
    pub role: UserRole,
}

impl<OTP, KV, AU, WU> FromRequestParts<HttpContext<OTP, KV, AU, WU>> for AuthUserId
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &HttpContext<OTP, KV, AU, WU>) -> Result<Self, Self::Rejection> {
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
