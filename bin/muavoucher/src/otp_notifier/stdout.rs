use crate::otp_notifier::OtpNotifierTrait;

#[derive(Debug, Clone, Default)]
pub struct StdoutOtpNotifier {}

impl OtpNotifierTrait for StdoutOtpNotifier {
    async fn send_to_email(&self, otp: &str, mail: &str) -> Result<(), super::OtpNotifierError> {
        log::info!("[StdoutOtpNotifier] otp for {mail} : {otp}");
        Ok(())
    }

    async fn send_to_phone(&self, otp: &str, phone: &str) -> Result<(), super::OtpNotifierError> {
        log::info!("[StdoutOtpNotifier] otp for {phone} : {otp}");
        Ok(())
    }
}
