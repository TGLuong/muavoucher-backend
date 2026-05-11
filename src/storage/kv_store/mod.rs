use std::fmt::Display;

use thiserror::Error;

pub mod memory;

#[derive(Debug, Error)]
pub enum KVStoreError {
    #[error("store reach limit")]
    ReachLimit,
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct OtpKey(pub String);

impl Display for OtpKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-otp", self.0)
    }
}

impl Into<String> for OtpKey {
    fn into(self) -> String {
        self.to_string()
    }
}

#[derive(Debug)]
pub struct CookieKey(pub String);

impl Display for CookieKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-cookie", self.0)
    }
}

impl Into<String> for CookieKey {
    fn into(self) -> String {
        self.to_string()
    }
}

pub trait KVStoreTrait: Clone + Sync + Send + 'static {
    fn set(&self, key: impl Into<String> + Send, value: impl Into<String> + Send) -> impl Future<Output = Result<(), KVStoreError>> + Send;
    fn get(&self, key: impl Into<String> + Send) -> impl Future<Output = Result<Option<String>, KVStoreError>> + Send;
    fn pop(&self, key: impl Into<String> + Send) -> impl Future<Output = Result<Option<String>, KVStoreError>> + Send;
}
