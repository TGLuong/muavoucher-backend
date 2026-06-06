use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
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
    #[error("Object Not Found: {0}")]
    NotFound(String),
}

#[derive(Debug, Default)]
pub enum OrderDirection {
    #[default]
    Desending,
    Acsending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PanigationRequest<O> {
    pub order: O,
    pub limit: u64,
    pub page: u64,
}

impl<O> Default for PanigationRequest<O>
where
    O: Default,
{
    fn default() -> Self {
        Self {
            order: Default::default(),
            limit: 20,
            page: 1,
        }
    }
}

#[derive(Debug)]
pub struct ListFilter<F, O> {
    pub filter: F,
    pub order: O,
    pub direction: OrderDirection,
    pub limit: u64,
    pub offset: u64,
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

    pub fn with_panigation(mut self, panigation: PanigationRequest<O>) -> Self {
        self.order = panigation.order;
        self.limit = panigation.limit;
        self.offset = (panigation.page - 1) * panigation.limit;
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
            limit: 20,
            offset: 0,
        }
    }
}
