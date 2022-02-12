use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait StreamProcessor {
  async fn recv(&self) -> Result<Message, ReceiveError>;
  async fn send(&self, message: SendInput);
}

#[derive(Debug)]
pub struct SendInput {
  pub topic: String,
  pub key: String,
  pub payload: Vec<u8>,
}

pub type ReceiveError = String;

#[derive(Debug)]
pub struct Message {
  pub key: Option<String>,
  pub topic: String,
  pub partition: i32,
  pub offset: i64,
  pub timestamp: Option<i64>,
}
