use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use num_format::{Locale, ToFormattedString};
use rust_decimal::{Decimal, prelude::FromPrimitive};

use crate::{
    auth_token::AuthTokenTrait,
    otp_notifier::OtpNotifierTrait,
    storage::kv_store::KVStoreTrait,
    transport::http::{
        context::HttpContext,
        request::link::UrlQuery,
        response::{ApiResponse, link::GetLinkResponse},
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
    Router::new().route("/api/v0/link", get(get_link)).with_state(context)
}

pub async fn get_link<OTP, KV, AU, WU>(
    State(context): State<HttpContext<OTP, KV, AU, WU>>,
    Query(query): Query<UrlQuery>,
) -> Result<Json<ApiResponse<GetLinkResponse>>, String>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    let mut response = GetLinkResponse::default();
    if query.url.contains("tiktok") {
        let tiktok_aff = context.logic.get_tiktok_link(query.url).await?;
        let tiktok_product = context.logic.get_tiktok_info(tiktok_aff.product_id).await?;
        let info = tiktok_product.products.first().cloned().ok_or("product not found".to_string())?;
        response.title = info.title;
        response.image_url = info.main_image_url;
        let minimum_amount = info.original_price.minimum_amount.parse::<u64>().map_err(|e| e.to_string())?;
        let commission = (minimum_amount / 100 * (info.commission.rate / 100)) / 100 * 80;
        response.price_text = format!("{}", minimum_amount.to_formatted_string(&Locale::vi));
        response.reward_text = format!("{}", commission.to_formatted_string(&Locale::vi));
        response.buy_url = tiktok_aff.affiliate_link;
        response.platform = "tiktok".to_string();
    } else {
        let product_info = context.logic.get_shopee_info(query.url.clone()).await?;
        let price = Decimal::from_u64(product_info.product_info.price).ok_or(format!("invalid price: {}", product_info.product_info.price))?;
        let link = context.logic.get_shopee_link(query.url.clone(), price).await?;
        response.title = product_info.product_info.product_name;
        response.image_url = product_info.product_info.image_url;
        response.price_text = product_info.product_info.price.to_formatted_string(&Locale::vi);
        let commission = product_info.product_info.commission / 100 * 80;
        response.reward_text = commission.to_formatted_string(&Locale::vi);
        response.buy_url = link;
    }
    Ok(Json(ApiResponse::success(Some("transaction successfully".into()), response)))
}
