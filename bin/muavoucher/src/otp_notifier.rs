use thiserror::Error;

pub mod stdout;
pub mod zalo_gmail;

#[derive(Debug, Error)]
pub enum OtpNotifierError {
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("{0:?}")]
    Letter(#[from] lettre::error::Error),
    #[error("{0:?}")]
    LetterTransport(#[from] lettre::transport::smtp::Error),
    #[error("email parse error: {0}")]
    EmailParser(String),
}

pub trait OtpNotifierTrait: Clone + Send + Sync + 'static {
    fn send_to_email(&self, otp: &str, mail: &str) -> impl Future<Output = Result<(), OtpNotifierError>> + Send;
    fn send_to_phone(&self, otp: &str, phone: &str) -> impl Future<Output = Result<(), OtpNotifierError>> + Send;
}
