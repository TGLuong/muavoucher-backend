use axum::{extract::FromRequestParts, http::request::Parts};
use reqwest::StatusCode;

use crate::{
    auth_token::AuthTokenTrait,
    otp_notifier::OtpNotifierTrait,
    storage::{entities::user::UserRole, kv_store::KVStoreTrait},
    transport::http::context::HttpContext,
    webhook_validator::WebhookValidator,
};

#[derive(Debug)]
pub struct ProductManagerUser;

impl<OTP, KV, AU, WU> FromRequestParts<HttpContext<OTP, KV, AU, WU>> for ProductManagerUser
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
        (role == UserRole::Admin || role == UserRole::ProductManager)
            .then_some(())
            .ok_or((StatusCode::UNAUTHORIZED, format!("claims {:?} is not admin", claims)))?;
        Ok(Self)
    }
}
