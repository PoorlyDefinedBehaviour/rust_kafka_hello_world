use contracts::stream_processor::{SendInput, StreamProcessor};
use tracing::{info, instrument};

pub struct CreateOrder {
  stream_processor: Box<dyn StreamProcessor>,
}

impl CreateOrder {
  pub fn new(stream_processor: Box<dyn StreamProcessor>) -> Self {
    Self { stream_processor }
  }
}

#[derive(Debug)]
pub struct CreateOrderInput {
  pub product_id: String,
  pub user_id: String,
  pub quantity: i64,
}

impl CreateOrder {
  #[instrument(skip(self))]
  pub async fn execute(&self, input: CreateOrderInput) {
    info!("publishing order event");

    self
      .stream_processor
      .send(SendInput {
        topic: String::from("orders"),
        key: String::from("order_placed"),
        payload: String::from("hello world").as_bytes().to_vec(),
      })
      .await;
  }
}
