use crate::{
    auth_token::AuthTokenTrait, otp_notifier::OtpNotifierTrait, storage::kv_store::KVStoreTrait, transport::http::context::HttpContext,
    webhook_validator::WebhookValidator,
};
use axum::{extract::FromRequestParts, http::request::Parts};

pub mod link;
pub mod product;
pub mod sepay;
pub mod user;

#[derive(Debug)]
pub struct OtpHeader(pub String);

impl<OTP, KV, AU, WU> FromRequestParts<HttpContext<OTP, KV, AU, WU>> for OtpHeader
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _state: &HttpContext<OTP, KV, AU, WU>) -> Result<Self, Self::Rejection> {
        let otp = parts
            .headers
            .get("user_otp")
            .ok_or("header missing user_otp".to_string())?
            .to_str()
            .map_err(|e| e.to_string())?;
        Ok(Self(otp.to_string()))
    }
}
