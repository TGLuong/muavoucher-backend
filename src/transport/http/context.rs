use crate::{auth_token::AuthTokenTrait, logic::Logic, otp_notifier::OtpNotifierTrait, storage::kv_store::KVStoreTrait};

#[derive(Debug, Clone)]
pub struct HttpContext<OTP, KV, AU>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
{
    pub logic: Logic<OTP, KV, AU>,
}

impl<OTP, KV, AU> HttpContext<OTP, KV, AU>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
{
    pub fn new(logic: Logic<OTP, KV, AU>) -> Self {
        Self { logic }
    }
}
