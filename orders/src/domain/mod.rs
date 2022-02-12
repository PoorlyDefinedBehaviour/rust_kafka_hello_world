use contracts::stream_processor::StreamProcessor;

use self::orders::commands::create_order::CreateOrder;

pub mod orders;

pub struct App {
  pub commands: Commands,
  pub queries: Queries,
}

impl App {
  pub fn new(stream_processor: Box<dyn StreamProcessor>) -> Self {
    Self {
      commands: Commands::new(stream_processor),
      queries: Queries::new(),
    }
  }
}

pub struct Commands {
  pub orders: OrderCommands,
}

impl Commands {
  pub fn new(stream_processor: Box<dyn StreamProcessor>) -> Self {
    Self {
      orders: OrderCommands {
        create_order: CreateOrder::new(stream_processor),
      },
    }
  }
}

pub struct OrderCommands {
  pub create_order: CreateOrder,
}

pub struct Queries;

impl Queries {
  pub fn new() -> Self {
    Queries
  }
}
