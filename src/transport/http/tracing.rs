use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{auth_token::AuthTokenTrait, otp_notifier::OtpNotifierTrait, storage::kv_store::KVStoreTrait, transport::http::context::HttpContext};

pub async fn tracing_handle_fn<OTP, KV, AU>(State(_state): State<HttpContext<OTP, KV, AU>>, request: Request, next: Next) -> Response
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
{
    let method = request.method().clone();
    let uri = request.uri().clone();
    let response = next.run(request).await;
    log::info!("[tracing_handle_fn] {} {} done: {}", uri, method, response.status());
    response
}
