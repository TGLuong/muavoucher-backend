use chrono::Utc;
use sea_orm::{ActiveValue::Set, prelude::*};
use uuid::Uuid;

use crate::storage::entities::cookie::{CookieEntity, CreateCookieRequest, UpdateCookieRequest};

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "cookie")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub area: Uuid,
    pub name: String,
    pub cookie: String,
    pub priority: i32,
    pub status: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl Into<CookieEntity> for Model {
    fn into(self) -> CookieEntity {
        CookieEntity {
            id: self.id,
            area: self.area,
            name: self.name,
            cookie: self.cookie,
            priority: self.priority,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::area::Entity", from = "Column::Area", to = "super::area::Column::Id")]
    Area,
}

impl Related<super::area::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Area.def()
    }
}

impl ActiveModel {
    pub fn patch(&mut self, request: UpdateCookieRequest) -> bool {
        let mut is_change = false;
        if let Some(area) = request.area {
            self.area = Set(area);
            is_change = true;
        }
        if let Some(cookie) = request.cookie {
            self.cookie = Set(cookie);
            self.status = Set(true);
            is_change = true;
        }
        if let Some(name) = request.name {
            self.name = Set(name);
            is_change = true;
        }
        if let Some(priority) = request.priority {
            self.priority = Set(priority);
            is_change = true;
        }
        if let Some(status) = request.status {
            self.status = Set(status);
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

impl From<CreateCookieRequest> for ActiveModel {
    fn from(value: CreateCookieRequest) -> Self {
        let id = Uuid::now_v7();
        Self {
            id: Set(id),
            area: Set(value.area),
            name: Set(value.name),
            cookie: Set(value.cookie),
            priority: Set(value.priority),
            status: Set(true),
            ..ActiveModel::new()
        }
    }
}
