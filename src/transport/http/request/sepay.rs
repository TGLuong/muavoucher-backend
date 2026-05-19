use std::vec;

use axum::{
    body::Bytes,
    extract::{FromRequest, FromRequestParts, Request},
    http::{StatusCode, request::Parts},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    auth_token::AuthTokenTrait, otp_notifier::OtpNotifierTrait, storage::kv_store::KVStoreTrait, transport::http::context::HttpContext,
    webhook_validator::WebhookValidator,
};

const HEADER_SIGNATURE: &str = "x-sepay-signature";
const HEADER_TIMESTAMP: &str = "x-sepay-timestamp";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransferType {
    In,
    Out,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SepayWebhookRequest {
    pub id: i32,
    pub gateway: String,
    pub transaction_date: String,
    pub account_number: String,
    pub sub_account: String,
    pub code: String,
    pub content: String,
    pub transfer_type: TransferType,
    pub description: String,
    pub transfer_amount: i32,
    pub accumulated: i32,
    pub reference_code: String,
}

pub struct SepayAuthenJson<T>(pub T);

impl<T, OTP, KV, AU, WU> FromRequest<HttpContext<OTP, KV, AU, WU>> for SepayAuthenJson<T>
where
    T: DeserializeOwned,
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: Request, state: &HttpContext<OTP, KV, AU, WU>) -> Result<Self, Self::Rejection> {
        let mut payload = vec![];
        let signature = request
            .headers()
            .get(HEADER_SIGNATURE)
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing or invalid signature header".into()))?
            .to_string();
        let timestamp = request
            .headers()
            .get(HEADER_TIMESTAMP)
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing or invalid timestamp header".into()))?
            .to_string();
        let body = Bytes::from_request(request, state)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Missing body".into()))?;
        payload.extend_from_slice(timestamp.as_bytes());
        payload.extend_from_slice(b".");
        payload.extend_from_slice(&body);
        state
            .logic
            .validate_sepay(&payload, &signature)
            .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;
        let body = serde_json::from_slice::<T>(&body).map_err(|e| (StatusCode::BAD_REQUEST, format!("parse body error: {e:?}")))?;
        Ok(Self(body))
    }
}
