use chrono::Utc;
use sea_orm::{ActiveValue::Set, prelude::*};
use uuid::Uuid;

use crate::storage::entities::area::{AreaEntity, CreateAreaRequest, UpdateAreaRequest};

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "area")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(column_type = "Decimal(Some((20, 2)))")]
    pub from: Decimal,
    #[sea_orm(column_type = "Decimal(Some((20, 2)))")]
    pub to: Decimal,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl Into<AreaEntity> for Model {
    fn into(self) -> AreaEntity {
        AreaEntity {
            id: self.id,
            name: self.name,
            from: self.from,
            to: self.to,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::cookie::Entity")]
    Cookie,
}

impl Related<super::area::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cookie.def()
    }
}

impl ActiveModel {
    pub fn patch(&mut self, request: UpdateAreaRequest) -> bool {
        let mut is_change = false;
        if let Some(name) = request.name {
            self.name = Set(name);
            is_change = true;
        }
        if let Some(from) = request.from {
            let from = from.round_dp(2);
            self.from = Set(from);
            is_change = true;
        }
        if let Some(to) = request.to {
            let to = to.round_dp(2);
            self.to = Set(to);
            is_change = true;
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

impl From<CreateAreaRequest> for ActiveModel {
    fn from(value: CreateAreaRequest) -> Self {
        let id = Uuid::now_v7();
        let from = value.from.round_dp(2);
        let to = value.to.round_dp(2);
        Self {
            id: Set(id),
            name: Set(value.name),
            from: Set(from),
            to: Set(to),
            ..ActiveModel::new()
        }
    }
}
