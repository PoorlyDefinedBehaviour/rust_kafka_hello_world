use async_trait::async_trait;
use contracts::mailer::{Email, MailerSendError};
use tracing::{info, instrument};

pub struct Mailer {}

impl Mailer {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl contracts::mailer::Mailer for Mailer {
  #[instrument(skip(self))]
  async fn send(&self, email: Email) -> Result<(), MailerSendError> {
    info!(from = ?email.from, to = ?email.to,"sending email");

    Ok(())
  }
}
