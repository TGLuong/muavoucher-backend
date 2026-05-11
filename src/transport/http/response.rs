use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<D> {
    pub code: u16,
    pub message: Option<String>,
    pub data: D,
}

impl<D> ApiResponse<D> {
    pub fn success(message: Option<String>, data: D) -> Self {
        Self { code: 200, message, data }
    }

    pub fn error(code: u16, message: Option<String>, data: D) -> Self {
        Self { code, message, data }
    }
}
