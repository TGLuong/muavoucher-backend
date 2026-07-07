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
        entities::product::{CreateProductRequest, ProductEntity, ProductFilter, ProductOrder, UpdateProductRequeset},
        kv_store::KVStoreTrait,
        repository::product::ProductRepositoryTrait,
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
        .route("/api/v0/product", get(list_product).post(create_product))
        .route("/api/v0/product/{id}", get(get_product).put(update_product).delete(delete_shop))
        .with_state(context)
}

pub async fn list_product<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    ProductManagerUser: ProductManagerUser,
    Query(filter): Query<ProductFilter>,
    Query(panigation): Query<PanigationRequest<ProductOrder>>,
) -> Result<Json<ApiResponse<PanigationRespose<ProductEntity>>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entities = context
        .database
        .product_repository
        .list(ListFilter::default().with_filter(filter).with_panigation(panigation.clone()))
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(
        Some("list product success".into()),
        PanigationRespose::default().with_items(entities).with_panigation(panigation),
    )))
}

pub async fn get_product<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    ProductManagerUser: ProductManagerUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProductEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context
        .database
        .product_repository
        .get(ProductFilter::default().with_id(id))
        .await
        .map_err(|e| e.to_string())?
        .ok_or(format!("product {id} not found"))?;
    Ok(Json(ApiResponse::success(Some("get product success".into()), entity)))
}

pub async fn create_product<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    ProductManagerUser: ProductManagerUser,
    Json(request): Json<CreateProductRequest>,
) -> Result<Json<ApiResponse<ProductEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context.database.product_repository.create(request).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("created product success".into()), entity)))
}

pub async fn update_product<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    ProductManagerUser: ProductManagerUser,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateProductRequeset>,
) -> Result<Json<ApiResponse<ProductEntity>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let entity = context.database.product_repository.update(id, request).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("update product success".into()), entity)))
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
    let count = context.database.product_repository.delete(vec![id]).await.map_err(|e| e.to_string())?;
    Ok(Json(ApiResponse::success(Some("delete area success".into()), count)))
}
