use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use uuid::Uuid;

use crate::{
    auth_token::AuthTokenTrait,
    otp_notifier::OtpNotifierTrait,
    storage::{
        ListFilter, PanigationRequest,
        entities::area::{AreaEntity, AreaFilter, AreaOrder, CreateAreaRequest, UpdateAreaRequest},
        kv_store::KVStoreTrait,
        repository::area::AreaRepositoryTrait,
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
        .route("/api/v0/area", get(list_area).post(create_area))
        .route("/api/v0/area/{id}", get(get_area).put(update_area).delete(delete_area))
        .with_state(context)
}

pub async fn list_area<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AdminUser: AdminUser,
    Query(filter): Query<AreaFilter>,
    Query(panigation): Query<PanigationRequest<AreaOrder>>,
) -> Result<Json<ApiResponse<PanigationRespose<AreaEntity>>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entities = context
        .database
        .area_repository
        .list(ListFilter::default().with_filter(filter).with_panigation(panigation.clone()))
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(
        Some("list area success".into()),
        PanigationRespose::default().with_items(entities).with_panigation(panigation),
    )))
}

pub async fn get_area<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AdminUser: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AreaEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context
        .database
        .area_repository
        .get(AreaFilter::default().with_id(id))
        .await
        .map_err(|e| e.to_string())?
        .ok_or(format!("area {id} not found"))?;
    Ok(Json(ApiResponse::success(Some("get area success".into()), entity)))
}

pub async fn create_area<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AdminUser: AdminUser,
    Json(request): Json<CreateAreaRequest>,
) -> Result<Json<ApiResponse<AreaEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context.database.area_repository.create(request).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("created area success".into()), entity)))
}

pub async fn update_area<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    AdminUser: AdminUser,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateAreaRequest>,
) -> Result<Json<ApiResponse<AreaEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context.database.area_repository.update(id, request).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("update area success".into()), entity)))
}

pub async fn delete_area<OTP, KV, AU, WU>(
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
    let count = context.database.area_repository.delete(vec![id]).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("delete area success".into()), count)))
}
