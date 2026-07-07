use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect, TryIntoModel, prelude::Expr,
};
use uuid::Uuid;

use crate::storage::{
    DatabaseError, ListFilter,
    entities::product::{CreateProductRequest, ProductEntity, ProductFilter, ProductOrder, UpdateProductRequeset},
    models::product,
};

pub trait ProductRepositoryTrait: Clone + Send + Sync + 'static {
    fn create(&self, request: CreateProductRequest) -> impl Future<Output = Result<ProductEntity, DatabaseError>> + Send;
    fn get(&self, filter: ProductFilter) -> impl Future<Output = Result<Option<ProductEntity>, DatabaseError>> + Send;
    fn list(&self, filter: ListFilter<ProductFilter, ProductOrder>) -> impl Future<Output = Result<Vec<ProductEntity>, DatabaseError>> + Send;
    fn update(&self, id: Uuid, request: UpdateProductRequeset) -> impl Future<Output = Result<ProductEntity, DatabaseError>> + Send;
    fn delete(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<u64, DatabaseError>> + Send;
}

#[derive(Debug, Clone)]
pub struct ProductRepository {
    conn: DatabaseConnection,
}

impl ProductRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub fn build_query<Q>(&self, mut query: Q, list_filter: ListFilter<ProductFilter, ProductOrder>) -> Q
    where
        Q: QueryFilter + QuerySelect + QueryOrder,
    {
        if let Some(id) = list_filter.filter.id {
            query = query.filter(product::Column::Id.eq(id));
        }
        if let Some(shop_id) = list_filter.filter.shop_id {
            query = query.filter(product::Column::ShopId.eq(shop_id));
        }
        if let Some(name) = list_filter.filter.name {
            query = query.filter(product::Column::Name.eq(name));
        }
        if let Some(search) = list_filter.filter.search {
            query = query.filter(Expr::cust_with_values("unaccent(name) ILIKE unaccent($1)", vec![format!("%{search}%")]));
        }
        match (list_filter.order, list_filter.direction) {
            (ProductOrder::CreatedAt, crate::storage::OrderDirection::Desending) => query = query.order_by(product::Column::CreatedAt, Order::Desc),
            (ProductOrder::CreatedAt, crate::storage::OrderDirection::Acsending) => query = query.order_by(product::Column::CreatedAt, Order::Asc),
        }
        query
    }
}

impl ProductRepositoryTrait for ProductRepository {
    async fn create(&self, request: CreateProductRequest) -> Result<ProductEntity, DatabaseError> {
        let active_model: product::ActiveModel = request.into();
        let model = active_model.insert(&self.conn).await?;
        Ok(model.into())
    }

    async fn get(&self, filter: ProductFilter) -> Result<Option<ProductEntity>, DatabaseError> {
        let mut query = product::Entity::find();
        query = self.build_query(query, ListFilter::default().with_filter(filter));
        let model = query.one(&self.conn).await?;
        Ok(model.map(|e| e.into()))
    }

    async fn list(&self, filter: ListFilter<ProductFilter, ProductOrder>) -> Result<Vec<ProductEntity>, DatabaseError> {
        let mut query = product::Entity::find();
        query = self.build_query(query, filter);
        let models = query.all(&self.conn).await?;
        Ok(models.into_iter().map(|e| e.into()).collect())
    }

    async fn update(&self, id: Uuid, request: UpdateProductRequeset) -> Result<ProductEntity, DatabaseError> {
        let model = product::Entity::find_by_id(id)
            .one(&self.conn)
            .await?
            .ok_or(DatabaseError::NotFound(format!("Shop {id} not found")))?;
        let mut active_model: product::ActiveModel = model.into();
        let is_change = active_model.patch(request);
        if is_change {
            active_model = active_model.save(&self.conn).await?;
        }
        Ok(active_model.try_into_model()?.into())
    }

    async fn delete(&self, ids: Vec<Uuid>) -> Result<u64, DatabaseError> {
        let mut rows_affected = 0;
        if !ids.is_empty() {
            let result = product::Entity::delete_many()
                .filter(product::Column::Id.is_in(ids))
                .exec(&self.conn)
                .await?;
            rows_affected = result.rows_affected;
        }
        Ok(rows_affected)
    }
}
