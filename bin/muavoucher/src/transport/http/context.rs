use crate::{
    auth_token::AuthTokenTrait,
    logic::Logic,
    otp_notifier::OtpNotifierTrait,
    storage::{kv_store::KVStoreTrait, repository::CenterDatabase},
    webhook_validator::WebhookValidator,
};

#[derive(Debug, Clone)]
pub struct HttpContext<OTP, KV, AU, WU>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    pub logic: Logic<OTP, KV, AU, WU>,
    pub database: CenterDatabase,
}

impl<OTP, KV, AU, WU> HttpContext<OTP, KV, AU, WU>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    pub fn new(logic: Logic<OTP, KV, AU, WU>, database: CenterDatabase) -> Self {
        Self { logic, database }
    }
}
