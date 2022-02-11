use async_trait::async_trait;

#[async_trait]
pub trait Notify {}

#[derive(Debug, PartialEq)]
enum Message {
  Sent {},
}
