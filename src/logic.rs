use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rand::RngExt;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    auth_token::{AuthTokenError, AuthTokenTrait, Claims},
    otp_notifier::{OtpNotifierError, OtpNotifierTrait},
    storage::{
        DatabaseError,
        entities::user::{CreateUserRequest, LoginUserRequest, LoginUserResponse, UserEntity, UserFilter, UserOtpType},
        kv_store::{KVStoreError, KVStoreTrait, OtpKey},
        repository::{CenterDatabase, user::UserRepositoryTrait},
    },
    webhook_validator::WebhookValidator,
};

#[derive(Debug, Error)]
pub enum LogicError {
    #[error("{0:?}")]
    OtpNotifier(#[from] OtpNotifierError),
    #[error("{0:?}")]
    Database(#[from] DatabaseError),
    #[error("{0:?}")]
    KVStore(#[from] KVStoreError),
    #[error("{0:?}")]
    AuthToken(#[from] AuthTokenError),
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("otp validation error")]
    OtpValidation,
    #[error("authentication error: {0}")]
    Authentication(String),
}

#[derive(Debug, Clone)]
pub struct Logic<OTP, KV, AU, WU> {
    otp_notifier: OTP,
    kv_store: KV,
    auth_token: AU,
    webhook_validator: WU,
    database: CenterDatabase,
}

impl<OTP, KV, AU, WU> Logic<OTP, KV, AU, WU> {
    pub fn new(otp_notifier: OTP, kv_store: KV, auth_token: AU, webhook_validator: WU, database: CenterDatabase) -> Self {
        Self {
            otp_notifier,
            kv_store,
            auth_token,
            webhook_validator,
            database,
        }
    }
}

impl<OTP, KV, AU, WU> Logic<OTP, KV, AU, WU>
where
    OTP: OtpNotifierTrait,
    KV: KVStoreTrait,
    AU: AuthTokenTrait,
    WU: WebhookValidator,
{
    pub async fn login_user(&self, request: LoginUserRequest) -> Result<LoginUserResponse, LogicError> {
        let mut filter = UserFilter::default();
        if let Some(email) = request.email {
            filter = filter.with_email(email);
        }
        if let Some(phone) = request.phone {
            filter = filter.with_phone(phone);
        }
        let user = self
            .database
            .user_repository
            .get(filter)
            .await?
            .ok_or(anyhow::anyhow!("user not found"))?;
        let hash = PasswordHash::new(&user.password).map_err(|e| anyhow::anyhow!("create PasswordHash error: {e:?}"))?;
        let argon2 = Argon2::default();
        argon2
            .verify_password(request.password.as_bytes(), &hash)
            .map_err(|e| anyhow::anyhow!("password invalid: {e:?}"))?;
        let id = user.id.to_string();
        let token = self.auth_token.generate(user.into())?;
        Ok(LoginUserResponse { token, id })
    }

    pub async fn create_user(&self, otp: String, request: CreateUserRequest) -> Result<UserEntity, LogicError> {
        let mut validate_otp = String::new();
        if let Some(phone) = request.phone.as_ref() {
            if let Some(data) = self.kv_store.pop(OtpKey(phone.clone())).await? {
                validate_otp = data;
            }
        }
        if let Some(email) = request.email.as_ref() {
            if let Some(data) = self.kv_store.pop(OtpKey(email.clone())).await? {
                validate_otp = data;
            }
        }
        (validate_otp == otp).then_some(());
        let entity = self.database.user_repository.create(request).await?;
        Ok(entity)
    }

    pub async fn get_user(&self, id: Uuid) -> Result<UserEntity, LogicError> {
        let entity = self
            .database
            .user_repository
            .get(UserFilter::default().with_id(id))
            .await?
            .ok_or(anyhow::anyhow!("user {id} not found"))?;
        Ok(entity)
    }

    pub async fn request_user_otp(&self, r#type: UserOtpType, data: String) -> Result<(), LogicError> {
        let otp = rand::rng().random_range(0..1000000);
        let otp = format!("{otp:06}");
        self.kv_store.set(OtpKey(data.clone()), otp.clone()).await?;
        match r#type {
            UserOtpType::Phone => self.otp_notifier.send_to_phone(&otp, &data).await?,
            UserOtpType::Email => self.otp_notifier.send_to_email(&otp, &data).await?,
        }
        Ok(())
    }

    pub async fn user_authen(&self, token: &str) -> Result<Claims, LogicError> {
        let claims = self.auth_token.validate(&token)?;
        Ok(claims)
    }

    pub fn validate_sepay(&self, payload: &[u8], signature: &str) -> Result<(), String> {
        self.webhook_validator.verify_signature(payload, signature).map_err(|e| e.to_string())
    }
}
