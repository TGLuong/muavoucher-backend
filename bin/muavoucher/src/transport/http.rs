use std::net::SocketAddr;

use axum::{Router, middleware};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::{
    auth_token::AuthTokenTrait,
    otp_notifier::OtpNotifierTrait,
    storage::kv_store::KVStoreTrait,
    transport::http::{
        api::{area, cookie, link, transaction, user},
        context::HttpContext,
        tracing::tracing_handle_fn,
    },
    webhook_validator::WebhookValidator,
};

pub mod api;
pub mod auth;
pub mod context;
pub mod request;
pub mod response;
pub mod tracing;

pub struct HttpServer {
    listener: TcpListener,
    router: Router,
}

impl HttpServer {
    pub async fn new<OTP, KV, AU, WU>(socket: SocketAddr, context: HttpContext<OTP, KV, AU, WU>) -> anyhow::Result<Self>
    where
        OTP: OtpNotifierTrait,
        KV: KVStoreTrait,
        AU: AuthTokenTrait,
        WU: WebhookValidator,
    {
        let router = Router::new()
            .merge(user::router(context.clone()))
            .merge(transaction::router(context.clone()))
            .merge(link::router(context.clone()))
            .merge(cookie::router(context.clone()))
            .merge(area::router(context.clone()))
            .layer(middleware::from_fn_with_state(context.clone(), tracing_handle_fn))
            .layer(CorsLayer::permissive());
        let listener = TcpListener::bind(socket).await?;
        Ok(Self { listener, router })
    }

    pub fn run(self) {
        tokio::spawn(async move { axum::serve(self.listener, self.router).await });
    }
}
