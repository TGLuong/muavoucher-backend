use std::sync::Arc;

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, address::AddressError, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};

use crate::otp_notifier::{OtpNotifierError, OtpNotifierTrait};

#[derive(Debug, Clone)]
pub struct ZaloGmailNotifier {
    gmail_sender: Arc<String>,
    gmail_key: Arc<String>,
    zalo_key: Arc<String>,
}

impl ZaloGmailNotifier {
    pub fn new(zalo_key: String, gmail_sender: String, gmail_key: String) -> Self {
        Self {
            gmail_sender: Arc::new(gmail_sender),
            gmail_key: Arc::new(gmail_key),
            zalo_key: Arc::new(zalo_key),
        }
    }
}

impl OtpNotifierTrait for ZaloGmailNotifier {
    async fn send_to_email(&self, otp: &str, email_address: &str) -> Result<(), super::OtpNotifierError> {
        let email_body = format!(
            r#"
            <html>
            <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                <p>Hello,</p>
                <p>Your one-time password (OTP) is:</p>
                <p style="font-size: 24px; font-weight: bold; color: #1a73e8; letter-spacing: 2px;">
                    {otp}
                </p>
                <p>This code is valid for the next <b>5 minutes</b>. Please do not share this code with anyone.</p>
                <br>
                <p>Best regards,<br>
                <strong>Muavoucher Team</strong></p>
            </body>
            </html>
            "#
        );
        let email = Message::builder()
            .from(
                format!("Muavoucher Support <{}>", self.gmail_sender)
                    .parse()
                    .map_err(|e: AddressError| OtpNotifierError::EmailParser(e.to_string()))?,
            )
            .to(email_address
                .parse()
                .map_err(|e: AddressError| OtpNotifierError::EmailParser(e.to_string()))?)
            .subject(format!("Your Verification Code: {otp}"))
            .header(ContentType::TEXT_HTML)
            .body(email_body)?;
        let creds = Credentials::new(self.gmail_sender.to_string(), self.gmail_key.to_string());
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        log::info!("[ZaloGmailNotifier] otp for {email_address} : {otp}");
        tokio::spawn(async move {
            match mailer.send(email).await {
                Ok(response) => log::info!("[ZaloGmailNotifier] mail notifier with response: {response:?}"),
                Err(err) => log::error!("[ZaloGmailNotifier] mail notifier with error: {err:?}"),
            }
        });
        Ok(())
    }

    async fn send_to_phone(&self, otp: &str, phone: &str) -> Result<(), super::OtpNotifierError> {
        Ok(())
    }
}
