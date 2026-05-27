use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLinkResponse {
    pub platform: String,
    pub title: String,
    pub image_url: String,
    pub price_text: String,
    pub reward_text: String,
    pub buy_url: String,
}

impl Default for GetLinkResponse {
    fn default() -> Self {
        Self {
            platform: "shopee".into(),
            title: Default::default(),
            image_url: Default::default(),
            price_text: Default::default(),
            reward_text: Default::default(),
            buy_url: Default::default(),
        }
    }
}
