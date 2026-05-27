use std::str::FromStr;

use thiserror::Error;
use uuid::Uuid;

use crate::proto::protocol::{ProductDataResponse, ProductRequest};

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("parser error: {0}")]
    Parser(String),
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductReq {
    pub id: Uuid,
    pub link: String,
}

impl Into<ProductRequest> for ProductReq {
    fn into(self) -> ProductRequest {
        ProductRequest {
            id: self.id.to_string(),
            link: self.link,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductDataRes {
    pub id: Uuid,
    pub name: String,
    pub price: (u64, Option<u64>),
    pub images: Vec<String>,
}

impl ProductDataRes {
    fn parse_price(price: &str) -> Option<(u64, Option<u64>)> {
        let clean_str = price.trim().replace('₫', "").replace('.', "");
        let mut parts = clean_str.split('-');
        let min = parts.next()?.trim().parse::<u64>().ok()?;
        let max = parts
            .next()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse::<u64>().ok());
        todo!()
    }
}

impl TryFrom<ProductDataResponse> for ProductDataRes {
    type Error = ProtocolError;

    fn try_from(value: ProductDataResponse) -> Result<Self, Self::Error> {
        let id = Uuid::from_str(&value.id).map_err(|e| ProtocolError::Parser(e.to_string()))?;
        let price = Self::parse_price(&value.price).ok_or(ProtocolError::Parser(format!("parse price error: {}", value.price)))?;
        Ok(Self {
            id,
            name: value.name,
            price,
            images: value.images,
        })
    }
}
