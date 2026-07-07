use chrono::Utc;
use sea_orm::{ActiveValue::Set, prelude::*};
use tower_http::follow_redirect::policy::PolicyExt;

use crate::storage::entities::product::{CreateProductRequest, ProductEntity, UpdateProductRequeset};

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub shop_id: Uuid,
    pub name: String,
    pub image: String,
    pub origin_link: String,
    pub affiliate_link: String,
    #[sea_orm(column_type = "Decimal(Some((20, 2)))")]
    pub price: Decimal,
    #[sea_orm(column_type = "Decimal(Some((20, 2)))")]
    pub commission: Decimal,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl Into<ProductEntity> for Model {
    fn into(self) -> ProductEntity {
        ProductEntity {
            id: self.id,
            shop_id: self.shop_id,
            image: self.image,
            origin_link: self.origin_link,
            affiliate_link: self.affiliate_link,
            price: self.price,
            commission: self.commission,
        }
    }
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModel {
    pub fn patch(&mut self, req: UpdateProductRequeset) -> bool {
        let mut is_change = false;
        if let Some(name) = req.name {
            self.name = Set(name);
            is_change = true;
        }
        if let Some(image) = req.image {
            self.image = Set(image);
            is_change = true;
        }
        if let Some(origin_link) = req.origin_link {
            self.origin_link = Set(origin_link);
            is_change = true;
        }
        if let Some(affiliate_link) = req.affiliate_link {
            self.affiliate_link = Set(affiliate_link);
            is_change = true;
        }
        if let Some(price) = req.price {
            self.price = Set(price);
            is_change = true;
        }
        if let Some(commission) = req.commission {
            self.commission = Set(commission);
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

impl From<CreateProductRequest> for ActiveModel {
    fn from(value: CreateProductRequest) -> Self {
        let id = Uuid::now_v7();
        Self {
            id: Set(id),
            shop_id: Set(value.shop_id),
            name: Set(value.name),
            image: Set(value.image),
            origin_link: Set(value.origin_link),
            affiliate_link: Set(value.affiliate_link),
            price: Set(value.price),
            commission: Set(value.commission),
            ..Self::new()
        }
    }
}
