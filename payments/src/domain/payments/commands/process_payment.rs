use std::sync::Arc;

use contracts::stream_processor::{SendInput, StreamProcessor};
use rand::Rng;
use tracing::instrument;

pub struct ProcessPayment {
  stream_processor: Arc<dyn StreamProcessor + Send + Sync>,
}

impl ProcessPayment {
  pub fn new(stream_processor: Arc<dyn StreamProcessor + Send + Sync>) -> Self {
    Self { stream_processor }
  }
}

impl ProcessPayment {
  #[instrument(skip(self))]
  pub async fn execute(&self) {
    let key = if rand::thread_rng().gen_bool(50.0) {
      String::from("payment_accepted")
    } else {
      String::from("payment_denied")
    };

    self
      .stream_processor
      .send(SendInput {
        topic: String::from("payments"),
        key,
        payload: String::from("hello world").as_bytes().to_vec(),
      })
      .await;
  }
}
