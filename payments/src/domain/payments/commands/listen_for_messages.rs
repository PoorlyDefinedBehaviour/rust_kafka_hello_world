use std::sync::Arc;

use contracts::stream_processor::StreamProcessor;

use tracing::{error, info, instrument};

use super::process_payment::ProcessPayment;

pub struct ListenForMessages {
  stream_processor: Arc<dyn StreamProcessor + Send + Sync>,
  process_payment: ProcessPayment,
}

impl ListenForMessages {
  pub fn new(
    stream_processor: Arc<dyn StreamProcessor + Send + Sync>,
    process_payment: ProcessPayment,
  ) -> Self {
    Self {
      stream_processor,
      process_payment,
    }
  }
}

impl ListenForMessages {
  #[instrument(skip(self))]
  pub async fn execute(self: Arc<Self>) {
    info!("waiting for messages");

    tokio::spawn(async move {
      loop {
        match self.stream_processor.recv().await {
          Err(e) => {
            error!("stream processor returned error. error={}", e);
          }
          Ok(message) => {
            info!("message received. message={:?}", &message);
            self.process_payment.execute().await;
          }
        }
      }
    });
  }
}
