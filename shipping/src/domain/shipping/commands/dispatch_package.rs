use std::sync::Arc;

use contracts::stream_processor::{SendInput, StreamProcessor};
use tracing::instrument;

pub struct DispatchPackage {
  stream_processor: Arc<dyn StreamProcessor + Send + Sync>,
}

impl DispatchPackage {
  pub fn new(stream_processor: Arc<dyn StreamProcessor + Send + Sync>) -> Self {
    Self { stream_processor }
  }
}

impl DispatchPackage {
  #[instrument(skip(self))]
  pub async fn execute(&self) {
    self
      .stream_processor
      .send(SendInput {
        topic: String::from("payments"),
        key: String::from("delivery_in_flight"),
        payload: String::from("hello world").as_bytes().to_vec(),
      })
      .await;
  }
}
