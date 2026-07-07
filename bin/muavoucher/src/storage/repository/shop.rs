use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect, TransactionTrait, TryIntoModel,
    prelude::Expr,
};
use uuid::Uuid;

use crate::storage::{
    DatabaseError, ListFilter,
    entities::shop::{CreateShopRequest, ShopEntity, ShopFilter, ShopOrder, UpdateShopRequest},
    models::{product, shop},
};

pub trait ShopRepositoryTrait: Clone + Send + Sync + 'static {
    fn create(&self, request: CreateShopRequest) -> impl Future<Output = Result<ShopEntity, DatabaseError>> + Send;
    fn get(&self, filter: ShopFilter) -> impl Future<Output = Result<Option<ShopEntity>, DatabaseError>> + Send;
    fn list(&self, filter: ListFilter<ShopFilter, ShopOrder>) -> impl Future<Output = Result<Vec<ShopEntity>, DatabaseError>> + Send;
    fn update(&self, id: Uuid, request: UpdateShopRequest) -> impl Future<Output = Result<ShopEntity, DatabaseError>> + Send;
    fn delete(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<u64, DatabaseError>> + Send;
}

#[derive(Debug, Clone)]
pub struct ShopRepository {
    conn: DatabaseConnection,
}

impl ShopRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub fn build_query<Q>(&self, mut query: Q, list_filter: ListFilter<ShopFilter, ShopOrder>) -> Q
    where
        Q: QueryFilter + QuerySelect + QueryOrder,
    {
        if let Some(id) = list_filter.filter.id {
            query = query.filter(shop::Column::Id.eq(id));
        }
        if let Some(name) = list_filter.filter.name {
            query = query.filter(shop::Column::Name.eq(name));
        }
        if let Some(search) = list_filter.filter.search {
            query = query.filter(Expr::cust_with_values("unaccent(name) ILIKE unaccent($1)", vec![format!("%{search}%")]));
        }
        match (list_filter.order, list_filter.direction) {
            (ShopOrder::CreatedAt, crate::storage::OrderDirection::Desending) => query = query.order_by(shop::Column::CreatedAt, Order::Desc),
            (ShopOrder::CreatedAt, crate::storage::OrderDirection::Acsending) => query = query.order_by(shop::Column::CreatedAt, Order::Asc),
        }
        query
    }
}

impl ShopRepositoryTrait for ShopRepository {
    async fn create(&self, request: CreateShopRequest) -> Result<ShopEntity, DatabaseError> {
        let active_model: shop::ActiveModel = request.into();
        let model = active_model.insert(&self.conn).await?;
        Ok(model.into())
    }

    async fn get(&self, filter: ShopFilter) -> Result<Option<ShopEntity>, DatabaseError> {
        let mut query = shop::Entity::find();
        query = self.build_query(query, ListFilter::default().with_filter(filter));
        let model = query.one(&self.conn).await?;
        Ok(model.map(|e| e.into()))
    }

    async fn list(&self, filter: ListFilter<ShopFilter, ShopOrder>) -> Result<Vec<ShopEntity>, DatabaseError> {
        let mut query = shop::Entity::find();
        query = self.build_query(query, filter);
        let models = query.all(&self.conn).await?;
        Ok(models.into_iter().map(|e| e.into()).collect())
    }

    async fn update(&self, id: Uuid, request: UpdateShopRequest) -> Result<ShopEntity, DatabaseError> {
        let model = shop::Entity::find_by_id(id)
            .one(&self.conn)
            .await?
            .ok_or(DatabaseError::NotFound(format!("Shop {id} not found")))?;
        let mut active_model: shop::ActiveModel = model.into();
        let is_change = active_model.patch(request);
        if is_change {
            active_model = active_model.save(&self.conn).await?;
        }
        Ok(active_model.try_into_model()?.into())
    }

    async fn delete(&self, ids: Vec<Uuid>) -> Result<u64, DatabaseError> {
        let mut rows_affected = 0;
        if !ids.is_empty() {
            let transaction = self.conn.begin().await?;
            let result = shop::Entity::delete_many()
                .filter(shop::Column::Id.is_in(ids.clone()))
                .exec(&transaction)
                .await?;
            product::Entity::delete_many()
                .filter(product::Column::ShopId.is_in(ids))
                .exec(&transaction)
                .await?;
            transaction.commit().await?;
            rows_affected = result.rows_affected;
        }
        Ok(rows_affected)
    }
}
