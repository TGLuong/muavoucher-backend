use axum::{Json, Router, extract::State, routing::post};
use serde_json::Value;

use crate::{
    auth_token::AuthTokenTrait,
    otp_notifier::OtpNotifierTrait,
    storage::kv_store::KVStoreTrait,
    transport::http::{
        context::HttpContext,
        request::sepay::{SepayAuthenJson, SepayWebhookRequest},
        response::ApiResponse,
    },
    webhook_validator::WebhookValidator,
};

pub fn router<OTP, KV, AU, WU>(context: HttpContext<OTP, KV, AU, WU>) -> Router
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    Router::new()
        .route("/api/v0/transaction/hook/sepay", post(sepay_hook))
        .route("/api/v0/transaction", post(create))
        .with_state(context)
}

pub async fn create<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    Json(request): Json<Value>,
) -> Result<Json<ApiResponse<()>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    log::info!("request: {request:?}");
    Ok(Json(ApiResponse::success(Some("transaction successfully".into()), ())))
}

pub async fn sepay_hook<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    SepayAuthenJson(request): SepayAuthenJson<SepayWebhookRequest>,
) -> Result<Json<ApiResponse<()>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    log::info!("request: {request:?}");
    Ok(Json(ApiResponse::success(Some("transaction successfully".into()), ())))
}
