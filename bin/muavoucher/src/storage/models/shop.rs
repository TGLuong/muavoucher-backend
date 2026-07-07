use chrono::Utc;
use sea_orm::{ActiveValue::Set, prelude::*};

use crate::storage::entities::shop::{CreateShopRequest, ShopEntity, UpdateShopRequest};

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "shop")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl Into<ShopEntity> for Model {
    fn into(self) -> ShopEntity {
        ShopEntity {
            id: self.id,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModel {
    pub fn patch(&mut self, req: UpdateShopRequest) -> bool {
        let mut is_change = false;
        if let Some(name) = req.name {
            self.name = Set(name);
            is_change = true;
        }
        if let Some(description) = req.description {
            self.description = Set(Some(description));
            is_change = true
        }
        is_change
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut model = self;
        if insert {
            model.created_at = Set(Utc::now().into())
        } else {
            model.updated_at = Set(Utc::now().into())
        }
        Ok(model)
    }
}

impl From<CreateShopRequest> for ActiveModel {
    fn from(value: CreateShopRequest) -> Self {
        let id = Uuid::now_v7();
        Self {
            id: Set(id),
            name: Set(value.name),
            description: Set(value.description),
            ..Self::new()
        }
    }
}
