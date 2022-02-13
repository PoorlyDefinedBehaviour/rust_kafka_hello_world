use std::sync::Arc;

use contracts::stream_processor::StreamProcessor;

use self::shipping::commands::{
  dispatch_package::DispatchPackage, listen_for_messages::ListenForMessages,
};

mod shipping;

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
  pub shipping: ShippingCommands,
}

impl Commands {
  pub fn new(stream_processor: Arc<dyn StreamProcessor + Send + Sync>) -> Self {
    Self {
      shipping: ShippingCommands {
        listen_for_messages: Arc::new(ListenForMessages::new(
          Arc::clone(&stream_processor),
          DispatchPackage::new(Arc::clone(&stream_processor)),
        )),
      },
    }
  }
}

pub struct ShippingCommands {
  pub listen_for_messages: Arc<ListenForMessages>,
}

pub struct Queries;

impl Queries {
  pub fn new() -> Self {
    Queries
  }
}
