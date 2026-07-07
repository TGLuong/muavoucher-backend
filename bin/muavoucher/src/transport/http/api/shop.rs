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
        entities::shop::{CreateShopRequest, ShopEntity, ShopFilter, ShopOrder, UpdateShopRequest},
        kv_store::KVStoreTrait,
        repository::shop::ShopRepositoryTrait,
    },
    transport::http::{
        context::HttpContext,
        request::product::ProductManagerUser,
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
        .route("/api/v0/shop", get(list_shop).post(create_shop))
        .route("/api/v0/shop/{id}", get(get_shop).put(update_shop).delete(delete_shop))
        .with_state(context)
}

pub async fn list_shop<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    ProductManagerUser: ProductManagerUser,
    Query(filter): Query<ShopFilter>,
    Query(panigation): Query<PanigationRequest<ShopOrder>>,
) -> Result<Json<ApiResponse<PanigationRespose<ShopEntity>>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entities = context
        .database
        .shop_repository
        .list(ListFilter::default().with_filter(filter).with_panigation(panigation.clone()))
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(
        Some("list area success".into()),
        PanigationRespose::default().with_items(entities).with_panigation(panigation),
    )))
}

pub async fn get_shop<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    ProductManagerUser: ProductManagerUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ShopEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context
        .database
        .shop_repository
        .get(ShopFilter::default().with_id(id))
        .await
        .map_err(|e| e.to_string())?
        .ok_or(format!("area {id} not found"))?;
    Ok(Json(ApiResponse::success(Some("get area success".into()), entity)))
}

pub async fn create_shop<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    ProductManagerUser: ProductManagerUser,
    Json(request): Json<CreateShopRequest>,
) -> Result<Json<ApiResponse<ShopEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context.database.shop_repository.create(request).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("created area success".into()), entity)))
}

pub async fn update_shop<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    ProductManagerUser: ProductManagerUser,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateShopRequest>,
) -> Result<Json<ApiResponse<ShopEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context.database.shop_repository.update(id, request).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("update area success".into()), entity)))
}

pub async fn delete_shop<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    ProductManagerUser: ProductManagerUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<u64>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let count = context.database.shop_repository.delete(vec![id]).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("delete area success".into()), count)))
}
