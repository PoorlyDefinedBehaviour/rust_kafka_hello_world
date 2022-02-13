use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Mailer {
  async fn send(&self, email: Email) -> Result<(), MailerSendError>;
}

pub type MailerSendError = String;

#[derive(Debug)]
pub struct Email {
  pub from: String,
  pub to: Vec<String>,
  pub subject: String,
  pub body: String,
  pub body_content_type: String,
}
