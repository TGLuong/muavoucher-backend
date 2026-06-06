use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CookieEntity {
    pub id: Uuid,
    pub area: Uuid,
    pub name: String,
    pub cookie: String,
    pub priority: i32,
    pub status: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCookieRequest {
    pub area: Uuid,
    pub name: String,
    pub cookie: String,
    pub priority: i32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateCookieRequest {
    pub area: Option<Uuid>,
    pub name: Option<String>,
    pub cookie: Option<String>,
    pub priority: Option<i32>,
    #[serde(skip)]
    pub status: Option<bool>,
}

impl UpdateCookieRequest {
    pub fn with_status(mut self, status: bool) -> Self {
        self.status = Some(status);
        self
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CookieFilter {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub search: Option<String>,
    pub priority: Option<i32>,
    pub status: Option<bool>,
}

impl CookieFilter {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum CookieOrder {
    #[default]
    CreatedAt,
}
