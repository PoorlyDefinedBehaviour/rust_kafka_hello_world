use std::sync::Arc;

use contracts::stream_processor::StreamProcessor;

use self::payments::commands::{
  listen_for_messages::ListenForMessages, process_payment::ProcessPayment,
};
mod payments;

pub struct App {
  pub commands: Commands,
  pub queries: Queries,
}

impl App {
  pub fn new(stream_processor: Arc<dyn StreamProcessor + Send + Sync>) -> Self {
    Self {
      commands: Commands::new(stream_processor),
      queries: Queries::new(),
    }
  }
}

pub struct Commands {
  pub payments: PaymentsCommands,
}

impl Commands {
  pub fn new(stream_processor: Arc<dyn StreamProcessor + Send + Sync>) -> Self {
    Self {
      payments: PaymentsCommands {
        listen_for_messages: Arc::new(ListenForMessages::new(
          Arc::clone(&stream_processor),
          ProcessPayment::new(Arc::clone(&stream_processor)),
        )),
      },
    }
  }
}

pub struct PaymentsCommands {
  pub listen_for_messages: Arc<ListenForMessages>,
}

pub struct Queries;

impl Queries {
  pub fn new() -> Self {
    Queries
  }
}
