use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    auth_token::AuthTokenTrait,
    otp_notifier::OtpNotifierTrait,
    storage::{
        ListFilter, PanigationRequest,
        entities::cookie::{CookieEntity, CookieFilter, CookieOrder, CreateCookieRequest, UpdateCookieRequest},
        kv_store::KVStoreTrait,
        repository::cookie::CookieRepositoryTrait,
    },
    transport::http::{
        context::HttpContext,
        request::user::AdminUser,
        response::{ApiResponse, panigation::PanigationRespose},
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
        .route("/api/v0/cookie", get(list_cookie).post(create_cookie))
        .route("/api/v0/cookie/{id}", get(get_cookie).put(update_cookie).delete(delete_cookie))
        .route("/api/v0/cookie/find/{price}", get(find_cookie))
        .with_state(context)
}

pub async fn list_cookie<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AdminUser: AdminUser,
    Query(filter): Query<CookieFilter>,
    Query(panigation): Query<PanigationRequest<CookieOrder>>,
) -> Result<Json<ApiResponse<PanigationRespose<CookieEntity>>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entities = context
        .database
        .cookie_repository
        .list(ListFilter::default().with_filter(filter).with_panigation(panigation.clone()))
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(
        Some("list cookie success".into()),
        PanigationRespose::default().with_items(entities).with_panigation(panigation),
    )))
}

pub async fn get_cookie<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AdminUser: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<CookieEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context
        .database
        .cookie_repository
        .get(CookieFilter::default().with_id(id))
        .await
        .map_err(|e| e.to_string())?
        .ok_or(format!("cookie {id} not found"))?;
    Ok(Json(ApiResponse::success(Some("get cookie success".into()), entity)))
}

pub async fn create_cookie<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AdminUser: AdminUser,
    Json(request): Json<CreateCookieRequest>,
) -> Result<Json<ApiResponse<CookieEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context.database.cookie_repository.create(request).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("created cookie success".into()), entity)))
}

pub async fn delete_cookie<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AdminUser: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<u64>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let count = context.database.cookie_repository.delete(vec![id]).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("updated cookie success".into()), count)))
}

pub async fn update_cookie<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AdminUser: AdminUser,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateCookieRequest>,
) -> Result<Json<ApiResponse<CookieEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context.database.cookie_repository.update(id, request).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("updated cookie success".into()), entity)))
}

pub async fn find_cookie<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AdminUser: AdminUser,
    Path(price): Path<Decimal>,
) -> Result<Json<ApiResponse<CookieEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let cookie = context.database.cookie_repository.find_cookie(price).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("find cookie success".into()), cookie)))
}
