use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect, TryIntoModel, sea_query::SimpleExpr,
};
use uuid::Uuid;

use crate::storage::{
    DatabaseError, ListFilter, OrderDirection,
    entities::user::{CreateUserRequest, UpdateUserRequest, UserEntity, UserFilter, UserOrder},
    models::user,
};

pub trait UserRepositoryTrait: Clone + Send + Sync + 'static {
    fn create(&self, request: CreateUserRequest) -> impl Future<Output = Result<UserEntity, DatabaseError>> + Send;
    fn get(&self, filter: UserFilter) -> impl Future<Output = Result<Option<UserEntity>, DatabaseError>> + Send;
    fn list(&self, filter: ListFilter<UserFilter, UserOrder>) -> impl Future<Output = Result<Vec<UserEntity>, DatabaseError>> + Send;
    fn update(&self, id: Uuid, request: UpdateUserRequest) -> impl Future<Output = Result<UserEntity, DatabaseError>> + Send;
    fn soft_delete(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<u64, DatabaseError>> + Send;
}

#[derive(Debug, Clone)]
pub struct UserRepository {
    conn: DatabaseConnection,
}

impl UserRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    fn query_builder<Q>(&self, mut query: Q, filter: ListFilter<UserFilter, UserOrder>) -> Q
    where
        Q: QueryFilter + QueryOrder + QuerySelect,
    {
        query = query.filter(user::Column::DeletedAt.is_null());
        if let Some(id) = filter.filter.id {
            query = query.filter(user::Column::Id.eq(id));
        }
        if let Some(full_name) = filter.filter.full_name {
            query = query.filter(user::Column::FullName.eq(full_name));
        }
        if let Some(phone) = filter.filter.phone {
            query = query.filter(user::Column::Phone.eq(phone));
        }
        if let Some(email) = filter.filter.email {
            query = query.filter(user::Column::Email.eq(email));
        }
        match (filter.order, filter.direction) {
            (UserOrder::CreatedAt, OrderDirection::Desending) => query = query.order_by(user::Column::CreatedAt, Order::Desc),
            (UserOrder::CreatedAt, OrderDirection::Acsending) => query = query.order_by(user::Column::CreatedAt, Order::Desc),
        }
        query = query.limit(filter.limit);
        query = query.offset(filter.offset);
        query
    }
}

impl UserRepositoryTrait for UserRepository {
    async fn create(&self, request: CreateUserRequest) -> Result<UserEntity, DatabaseError> {
        let active_model: user::ActiveModel = request.try_into()?;
        let model = active_model.insert(&self.conn).await?;
        Ok(model.into())
    }

    async fn get(&self, filter: UserFilter) -> Result<Option<UserEntity>, DatabaseError> {
        let mut query = user::Entity::find();
        query = self.query_builder(query, ListFilter::default().with_filter(filter));
        let model = query.one(&self.conn).await?;
        Ok(model.map(|e| e.into()))
    }

    async fn list(&self, filter: ListFilter<UserFilter, UserOrder>) -> Result<Vec<UserEntity>, DatabaseError> {
        let mut query = user::Entity::find();
        query = self.query_builder(query, filter);
        let entities = query.all(&self.conn).await?.into_iter().map(|e| e.into()).collect();
        Ok(entities)
    }

    async fn update(&self, id: Uuid, request: UpdateUserRequest) -> Result<UserEntity, DatabaseError> {
        let model = user::Entity::find_by_id(id)
            .one(&self.conn)
            .await?
            .ok_or(anyhow::anyhow!("user with id {id} not found"))?;
        let mut active_model: user::ActiveModel = model.into();
        active_model.patch(request);
        let active_model = active_model.save(&self.conn).await?;
        Ok(active_model.try_into_model()?.into())
    }

    async fn soft_delete(&self, ids: Vec<Uuid>) -> Result<u64, DatabaseError> {
        let query = user::Entity::update_many()
            .filter(user::Column::Id.is_in(ids))
            .col_expr(user::Column::DeletedAt, SimpleExpr::Value(Utc::now().into()));
        let result = query.exec(&self.conn).await?;
        Ok(result.rows_affected)
    }
}
