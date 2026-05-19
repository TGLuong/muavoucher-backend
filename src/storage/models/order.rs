use chrono::Utc;
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub order_data: chrono::DateTime<Utc>,
    pub product_name: String,
    pub order_code: String,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub order_value: Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub commission: Decimal,
    pub status: String,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity", from = "Column::UserId", to = "super::user::Column::Id")]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
