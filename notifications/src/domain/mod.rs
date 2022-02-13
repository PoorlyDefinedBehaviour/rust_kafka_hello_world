use std::sync::Arc;

use contracts::{mailer::Mailer, stream_processor::StreamProcessor};

use self::notifications::commands::{
  listen_for_messages::ListenForMessages, send_email_notification::SendEmailNotification,
};

pub mod notifications;

pub struct App {
  pub commands: Commands,
  pub queries: Queries,
}

impl App {
  pub fn new(
    mailer: Box<dyn Mailer>,
    stream_processor: Arc<dyn StreamProcessor + Send + Sync>,
  ) -> Self {
    App {
      commands: Commands::new(mailer, stream_processor),
      queries: Queries::new(),
    }
  }
}

pub struct Commands {
  pub notifications: NotificationCommands,
}

impl Commands {
  pub fn new(
    mailer: Box<dyn Mailer>,
    stream_processor: Arc<dyn StreamProcessor + Send + Sync>,
  ) -> Self {
    Commands {
      notifications: NotificationCommands {
        send_email_notification: SendEmailNotification::new(mailer),
        listen_for_messages: Arc::new(ListenForMessages::new(stream_processor)),
      },
    }
  }
}

pub struct NotificationCommands {
  pub send_email_notification: SendEmailNotification,
  pub listen_for_messages: Arc<ListenForMessages>,
}

pub struct Queries {}

impl Queries {
  pub fn new() -> Self {
    Self {}
  }
}
