use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect, TryIntoModel, prelude::Expr,
};
use uuid::Uuid;

use crate::storage::{
    DatabaseError, ListFilter,
    entities::area::{AreaEntity, AreaFilter, AreaOrder, CreateAreaRequest, UpdateAreaRequest},
    models::area,
};

pub trait AreaRepositoryTrait: Clone + Send + Sync + 'static {
    fn create(&self, request: CreateAreaRequest) -> impl Future<Output = Result<AreaEntity, DatabaseError>> + Send;
    fn get(&self, filter: AreaFilter) -> impl Future<Output = Result<Option<AreaEntity>, DatabaseError>> + Send;
    fn list(&self, filter: ListFilter<AreaFilter, AreaOrder>) -> impl Future<Output = Result<Vec<AreaEntity>, DatabaseError>> + Send;
    fn update(&self, id: Uuid, request: UpdateAreaRequest) -> impl Future<Output = Result<AreaEntity, DatabaseError>> + Send;
    fn delete(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<u64, DatabaseError>> + Send;
}

#[derive(Debug, Clone)]
pub struct AreaRepository {
    conn: DatabaseConnection,
}

impl AreaRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    fn query_builder<Q>(&self, mut query: Q, list_filter: ListFilter<AreaFilter, AreaOrder>) -> Q
    where
        Q: QueryFilter + QuerySelect + QueryOrder,
    {
        if let Some(id) = list_filter.filter.id {
            query = query.filter(area::Column::Id.eq(id));
        }
        if let Some(name) = list_filter.filter.name {
            query = query.filter(area::Column::Name.eq(name));
        }
        if let Some(search) = list_filter.filter.search {
            query = query.filter(Expr::cust_with_values("unaccent(name) ILIKE unaccent($1)", vec![format!("%{search}%")]));
        }
        match (list_filter.order, list_filter.direction) {
            (AreaOrder::CreatedAt, crate::storage::OrderDirection::Desending) => query = query.order_by(area::Column::CreatedAt, Order::Desc),
            (AreaOrder::CreatedAt, crate::storage::OrderDirection::Acsending) => query = query.order_by(area::Column::CreatedAt, Order::Asc),
        }
        query
    }
}

impl AreaRepositoryTrait for AreaRepository {
    async fn create(&self, request: CreateAreaRequest) -> Result<AreaEntity, DatabaseError> {
        let active_model: area::ActiveModel = request.into();
        let model = active_model.insert(&self.conn).await?;
        Ok(model.into())
    }

    async fn get(&self, filter: AreaFilter) -> Result<Option<AreaEntity>, DatabaseError> {
        let mut query = area::Entity::find();
        query = self.query_builder(query, ListFilter::default().with_filter(filter));
        let model = query.one(&self.conn).await?;
        Ok(model.map(|e| e.into()))
    }

    async fn list(&self, filter: ListFilter<AreaFilter, AreaOrder>) -> Result<Vec<AreaEntity>, DatabaseError> {
        let mut query = area::Entity::find();
        query = self.query_builder(query, filter);
        let entities = query.all(&self.conn).await?.into_iter().map(|e| e.into()).collect();
        Ok(entities)
    }

    async fn update(&self, id: Uuid, request: UpdateAreaRequest) -> Result<AreaEntity, DatabaseError> {
        let model = area::Entity::find_by_id(id)
            .one(&self.conn)
            .await?
            .ok_or(DatabaseError::NotFound(format!("Area {id} not found")))?;
        let mut active_model: area::ActiveModel = model.into();
        let is_change = active_model.patch(request);
        if is_change {
            active_model = active_model.save(&self.conn).await?;
        }
        Ok(active_model.try_into_model()?.into())
    }

    async fn delete(&self, ids: Vec<Uuid>) -> Result<u64, DatabaseError> {
        let mut rows_affected = 0;
        if !ids.is_empty() {
            let result = area::Entity::delete_many().filter(area::Column::Id.is_in(ids)).exec(&self.conn).await?;
            rows_affected = result.rows_affected;
        }
        Ok(rows_affected)
    }
}
