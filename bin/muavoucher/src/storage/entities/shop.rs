use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ShopEntity {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShopRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateShopRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ShopFilter {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub search: Option<String>,
}

impl ShopFilter {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ShopOrder {
    #[default]
    CreatedAt,
}
