use contracts::mailer::{Email, Mailer, MailerSendError};
use tracing::instrument;

pub struct SendEmailNotification {
  mailer: Box<dyn Mailer>,
}

impl SendEmailNotification {
  pub fn new(mailer: Box<dyn Mailer>) -> Self {
    Self { mailer }
  }
}

impl SendEmailNotification {
  #[instrument(skip(self))]
  pub async fn execute(&self, email: Email) -> Result<(), MailerSendError> {
    let _ = self.mailer.send(email).await;
    Ok(())
  }
}
