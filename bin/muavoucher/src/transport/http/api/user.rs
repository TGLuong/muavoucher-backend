use axum::{
    Json, Router,
    extract::{Path, State},
    middleware,
    routing::{get, post},
};

use crate::{
    auth_token::AuthTokenTrait,
    otp_notifier::OtpNotifierTrait,
    storage::{
        entities::user::{CreateUserRequest, GetUserResponse, LoginUserRequest, LoginUserResponse, UserEntity, UserOtpType},
        kv_store::KVStoreTrait,
    },
    transport::http::{context::HttpContext, request::user::AuthUserId, response::ApiResponse, tracing::tracing_handle_fn},
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
        .route("/api/v0/user/{id}", get(get_user))
        .route("/api/v0/user/login", post(login))
        .route("/api/v0/user/signup", post(signup))
        .route("/api/v0/user/otp/{type}/{data}", post(user_otp))
        .route("/api/v0/user/forgot", post(forgot))
        .route("/api/v0/user/change-password", post(change_password))
        .with_state(context.clone())
        .layer(middleware::from_fn_with_state(context, tracing_handle_fn))
}

pub async fn get_user<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AuthUserId { id, role: _ }: AuthUserId,
) -> Result<Json<ApiResponse<GetUserResponse>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let user = context.logic.get_user(id).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(None, user.into())))
}

pub async fn login<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    Json(request): Json<LoginUserRequest>,
) -> Result<Json<ApiResponse<LoginUserResponse>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let response = context.logic.login_user(request).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("user login successfully".into()), response)))
}

pub async fn signup<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    // OtpHeader(otp): OtpHeader,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let user = context.logic.create_user("".into(), request).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("create user success".into()), user)))
}

pub async fn user_otp<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    Path((r#type, data)): Path<(UserOtpType, String)>,
) -> Result<Json<ApiResponse<()>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    context.logic.request_user_otp(r#type, data).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("ok".into()), ())))
}

pub async fn forgot<OTP, KV, AU, WU>(State(context): State<HttpContext<OTP, KV, AU, WU>>) -> String
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    "forgot".into()
}

pub async fn change_password<OTP, KV, AU, WU>(State(context): State<HttpContext<OTP, KV, AU, WU>>) -> String
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    "forgot".into()
}
