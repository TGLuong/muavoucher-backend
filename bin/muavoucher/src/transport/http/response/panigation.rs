use serde::{Deserialize, Serialize};

use crate::storage::PanigationRequest;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PanigationRespose<D> {
    pub items: Vec<D>,
    pub limit: u64,
    pub page: u64,
}

impl<D> PanigationRespose<D> {
    pub fn with_items(mut self, items: Vec<D>) -> Self {
        self.items = items;
        self
    }

    pub fn with_panigation<O>(mut self, panigation: PanigationRequest<O>) -> Self {
        self.limit = panigation.limit;
        self.page = panigation.page;
        self
    }
}
