use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, Order, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
    TryIntoModel, prelude::Expr,
};
use uuid::Uuid;

use crate::storage::{
    DatabaseError, ListFilter,
    entities::cookie::{CookieEntity, CookieFilter, CookieOrder, CreateCookieRequest, UpdateCookieRequest},
    models::{area, cookie},
};

pub trait CookieRepositoryTrait: Clone + Send + Sync + 'static {
    fn create(&self, request: CreateCookieRequest) -> impl Future<Output = Result<CookieEntity, DatabaseError>> + Send;
    fn get(&self, filter: CookieFilter) -> impl Future<Output = Result<Option<CookieEntity>, DatabaseError>> + Send;
    fn find_cookie(&self, price: Decimal) -> impl Future<Output = Result<CookieEntity, DatabaseError>> + Send;
    fn list(&self, filter: ListFilter<CookieFilter, CookieOrder>) -> impl Future<Output = Result<Vec<CookieEntity>, DatabaseError>> + Send;
    fn update(&self, id: Uuid, request: UpdateCookieRequest) -> impl Future<Output = Result<CookieEntity, DatabaseError>> + Send;
    fn delete(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<u64, DatabaseError>> + Send;
}

#[derive(Debug, Clone)]
pub struct CookieRepository {
    conn: DatabaseConnection,
}

impl CookieRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    fn query_builder<Q>(&self, mut query: Q, list_filter: ListFilter<CookieFilter, CookieOrder>) -> Q
    where
        Q: QueryFilter + QuerySelect + QueryOrder,
    {
        if let Some(id) = list_filter.filter.id {
            query = query.filter(cookie::Column::Id.eq(id));
        }
        if let Some(area) = list_filter.filter.area {
            query = query.filter(cookie::Column::Area.eq(area));
        }
        if let Some(name) = list_filter.filter.name {
            query = query.filter(cookie::Column::Name.eq(name));
        }
        if let Some(search) = list_filter.filter.search {
            query = query.filter(Expr::cust_with_values("unaccent(name) ILIKE unaccent($1)", vec![format!("%{search}%")]));
        }
        if let Some(priority) = list_filter.filter.priority {
            query = query.filter(cookie::Column::Priority.eq(priority));
        }
        if let Some(status) = list_filter.filter.status {
            query = query.filter(cookie::Column::Status.eq(status));
        }
        query = query.order_by(cookie::Column::Priority, Order::Asc);
        match (list_filter.order, list_filter.direction) {
            (CookieOrder::CreatedAt, crate::storage::OrderDirection::Desending) => query = query.order_by(cookie::Column::CreatedAt, Order::Desc),
            (CookieOrder::CreatedAt, crate::storage::OrderDirection::Acsending) => query = query.order_by(cookie::Column::CreatedAt, Order::Asc),
        }
        query
    }
}

impl CookieRepositoryTrait for CookieRepository {
    async fn create(&self, request: CreateCookieRequest) -> Result<CookieEntity, DatabaseError> {
        let active_model: cookie::ActiveModel = request.into();
        let model = active_model.insert(&self.conn).await?;
        Ok(model.into())
    }

    async fn get(&self, filter: CookieFilter) -> Result<Option<CookieEntity>, DatabaseError> {
        let mut query = cookie::Entity::find();
        query = self.query_builder(query, ListFilter::default().with_filter(filter));
        let model = query.one(&self.conn).await?;
        Ok(model.map(|e| e.into()))
    }

    async fn find_cookie(&self, price: Decimal) -> Result<CookieEntity, DatabaseError> {
        let result = cookie::Entity::find()
            .join(JoinType::InnerJoin, cookie::Relation::Area.def())
            .filter(area::Column::From.lt(price))
            .filter(area::Column::To.gt(price))
            .filter(cookie::Column::Status.eq(true))
            .order_by(cookie::Column::Priority, Order::Asc)
            .one(&self.conn)
            .await?
            .ok_or(DatabaseError::NotFound(format!("can not find cookie for price {price}")))?;
        Ok(result.into())
    }

    async fn list(&self, filter: ListFilter<CookieFilter, CookieOrder>) -> Result<Vec<CookieEntity>, DatabaseError> {
        let mut query = cookie::Entity::find();
        query = self.query_builder(query, filter);
        let entities = query.all(&self.conn).await?.into_iter().map(|e| e.into()).collect();
        Ok(entities)
    }

    async fn update(&self, id: Uuid, request: UpdateCookieRequest) -> Result<CookieEntity, DatabaseError> {
        let model = cookie::Entity::find_by_id(id)
            .one(&self.conn)
            .await?
            .ok_or(DatabaseError::NotFound(format!("cookie {id} not found")))?;
        let mut active_model: cookie::ActiveModel = model.into();
        let is_change = active_model.patch(request);
        if is_change {
            active_model = active_model.save(&self.conn).await?;
        }
        Ok(active_model.try_into_model()?.into())
    }

    async fn delete(&self, ids: Vec<Uuid>) -> Result<u64, DatabaseError> {
        let mut rows_affected = 0;
        if !ids.is_empty() {
            let result = cookie::Entity::delete_many()
                .filter(cookie::Column::Id.is_in(ids))
                .exec(&self.conn)
                .await?;
            rows_affected = result.rows_affected;
        }
        Ok(rows_affected)
    }
}
