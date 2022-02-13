use std::sync::Arc;

use contracts::stream_processor::StreamProcessor;

use tracing::{error, info, instrument};

pub struct ListenForMessages {
  stream_processor: Arc<dyn StreamProcessor + Send + Sync>,
}

impl ListenForMessages {
  pub fn new(stream_processor: Arc<dyn StreamProcessor + Send + Sync>) -> Self {
    Self { stream_processor }
  }
}

impl ListenForMessages {
  #[instrument(skip(self))]
  pub async fn execute(self: Arc<Self>) -> ! {
    info!("waiting for messages");

    loop {
      match self.stream_processor.recv().await {
        Err(e) => {
          error!("stream processor returned error. error={}", e);
        }
        Ok(message) => {
          info!("message received. message={:?}", &message);
        }
      }
    }
  }
}
