use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait SendEmail {
  async fn send(email: Email);
}

#[derive(Debug)]
pub struct Email {
  from: String,
  to: Vec<String>,
  subject: String,
  body: String,
  body_content_type: String,
}

pub struct Mailer<T> {
  email_sender: T,
}

impl<T: SendEmail> Mailer<T> {
  pub fn new(email_sender: T) -> Self {
    Self { email_sender }
  }

  pub fn send(&mut self, email: Email) {}
}
