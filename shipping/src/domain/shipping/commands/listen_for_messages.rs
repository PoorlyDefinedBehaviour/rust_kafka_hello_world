use std::sync::Arc;

use contracts::stream_processor::StreamProcessor;

use tracing::{error, info, instrument};

use super::dispatch_package::DispatchPackage;

pub struct ListenForMessages {
  stream_processor: Arc<dyn StreamProcessor + Send + Sync>,
  dispatch_package: DispatchPackage,
}

impl ListenForMessages {
  pub fn new(
    stream_processor: Arc<dyn StreamProcessor + Send + Sync>,
    dispatch_package: DispatchPackage,
  ) -> Self {
    Self {
      stream_processor,
      dispatch_package,
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
            self.dispatch_package.execute().await;
          }
        }
      }
    });
  }
}
