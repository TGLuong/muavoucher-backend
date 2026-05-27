use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};

use crate::{
    auth_token::AuthTokenTrait, otp_notifier::OtpNotifierTrait, storage::kv_store::KVStoreTrait, transport::http::context::HttpContext,
    webhook_validator::WebhookValidator,
};

async fn ws_crawler<OTP, KV, AU, WU>(ws: WebSocketUpgrade, State(context): State<HttpContext<OTP, KV, AU, WU>>) -> impl IntoResponse
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    ws.on_upgrade(async move |socket| {})
}
