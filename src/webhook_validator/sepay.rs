use std::sync::Arc;

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

use crate::webhook_validator::{WebhookValidator, WebhookValidatorError};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct SepayWebhookValidator {
    key: Arc<String>,
}

impl SepayWebhookValidator {
    pub fn new(key: String) -> Self {
        Self { key: Arc::new(key) }
    }
}

impl WebhookValidator for SepayWebhookValidator {
    fn verify_signature(&self, payload: &[u8], signature: &str) -> Result<(), WebhookValidatorError> {
        let mut mac = HmacSha256::new_from_slice(self.key.as_bytes()).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        mac.update(payload);
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let validator = hex::encode(code_bytes);
        (&validator == signature).then_some(()).ok_or(WebhookValidatorError::SignatureInvalid)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::webhook_validator::{WebhookValidator, WebhookValidatorError, sepay::SepayWebhookValidator};

    #[test]
    fn accepts_valid_signature() {
        let validator = SepayWebhookValidator::new("secret".to_string());
        let signature = "88aab3ede8d3adf94d26ab90d3bafd4a2083070c3bcce9c014ee04a443847c0b";

        let result = validator.verify_signature(b"hello", signature);

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_signature() {
        let validator = SepayWebhookValidator::new("secret".to_string());

        let result = validator.verify_signature(b"hello", "invalid-signature");

        assert!(matches!(result, Err(WebhookValidatorError::SignatureInvalid)));
    }
}
