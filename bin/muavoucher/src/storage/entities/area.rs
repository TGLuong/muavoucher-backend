use chrono::Utc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AreaEntity {
    pub id: Uuid,
    pub name: String,
    pub from: Decimal,
    pub to: Decimal,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAreaRequest {
    pub name: String,
    pub from: Decimal,
    pub to: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAreaRequest {
    pub name: Option<String>,
    pub from: Option<Decimal>,
    pub to: Option<Decimal>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AreaFilter {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub search: Option<String>,
}

impl AreaFilter {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum AreaOrder {
    #[default]
    CreatedAt,
}
