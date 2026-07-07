use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProductEntity {
    pub id: Uuid,
    pub shop_id: Uuid,
    pub image: String,
    pub origin_link: String,
    pub affiliate_link: String,
    pub price: Decimal,
    pub commission: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub shop_id: Uuid,
    pub name: String,
    pub image: String,
    pub origin_link: String,
    pub affiliate_link: String,
    pub price: Decimal,
    pub commission: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequeset {
    pub name: Option<String>,
    pub image: Option<String>,
    pub origin_link: Option<String>,
    pub affiliate_link: Option<String>,
    pub price: Option<Decimal>,
    pub commission: Option<Decimal>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProductFilter {
    pub id: Option<Uuid>,
    pub shop_id: Option<Uuid>,
    pub name: Option<String>,
    pub search: Option<String>,
}

impl ProductFilter {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ProductOrder {
    #[default]
    CreatedAt,
}
