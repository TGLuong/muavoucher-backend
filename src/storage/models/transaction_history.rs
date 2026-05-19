use chrono::Utc;
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "transaction_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_number: String,
    pub r#type: String,
    pub message: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((20, 2)))")]
    pub transaction: Decimal,
    #[sea_orm(column_type = "Decimal(Some((20, 2)))")]
    pub balance: Decimal,
    pub reference_code: Option<String>,
    pub transaction_date: chrono::DateTime<Utc>,
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
