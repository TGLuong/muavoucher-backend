use thiserror::Error;

pub mod sepay;

#[derive(Debug, Error)]
pub enum WebhookValidatorError {
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("signature invalid")]
    SignatureInvalid,
}

pub trait WebhookValidator: Clone + Send + Sync + 'static {
    fn verify_signature(&self, payload: &[u8], signature: &str) -> Result<(), WebhookValidatorError>;
}
