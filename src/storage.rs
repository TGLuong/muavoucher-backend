use sea_orm::DbErr;
use thiserror::Error;

pub mod entities;
pub mod kv_store;
pub mod migration;
pub mod models;
pub mod repository;

pub type Storage = ();

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database error {0:?}")]
    Database(#[from] DbErr),
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Debug, Default)]
pub enum OrderDirection {
    #[default]
    Desending,
    Acsending,
}

#[derive(Debug)]
pub struct ListFilter<F, O> {
    filter: F,
    order: O,
    direction: OrderDirection,
    limit: u64,
    offset: u64,
}

impl<F, O> ListFilter<F, O> {
    pub fn with_filter(mut self, filter: F) -> Self {
        self.filter = filter;
        self
    }

    pub fn with_order(mut self, order: O) -> Self {
        self.order = order;
        self
    }

    pub fn with_direction(mut self, direction: OrderDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<F, O> Default for ListFilter<F, O>
where
    F: Default,
    O: Default,
{
    fn default() -> Self {
        Self {
            filter: Default::default(),
            order: Default::default(),
            direction: Default::default(),
            limit: 100,
            offset: 0,
        }
    }
}
