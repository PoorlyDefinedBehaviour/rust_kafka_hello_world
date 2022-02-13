use std::sync::Arc;

use contracts::stream_processor::StreamProcessor;
use domain::App;
use dotenv::dotenv;

mod domain;

#[tokio::main]
async fn main() {
  std::env::set_var(
    "RUST_LOG",
    std::env::var("RUST_LOG").unwrap_or_else(|_| format!("{}=trace", env!("CARGO_PKG_NAME"))),
  );

  dotenv().ok();

  let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());

  tracing_subscriber::fmt()
    .with_writer(move || non_blocking_writer.clone())
    .init();

  let stream_processor = infra::kafka::Kafka::new(infra::kafka::Config {
    group_id: env!("CARGO_PKG_NAME").to_string(),
    bootstrap_servers: std::env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap(),
    enable_partition_eof: std::env::var("KAFKA_ENABLE_PARTITION_EOF")
      .unwrap()
      .parse::<bool>()
      .unwrap(),
    session_timeout_ms: std::env::var("KAFKA_SESSION_TIMEOUT_MS")
      .unwrap()
      .parse::<usize>()
      .unwrap(),
    message_timeout_ms: std::env::var("KAFKA_MESSAGE_TIMEOUT_MS")
      .unwrap()
      .parse::<usize>()
      .unwrap(),
    enable_auto_commit: std::env::var("KAFKA_ENABLE_AUTO_COMMIT")
      .unwrap()
      .parse::<bool>()
      .unwrap(),
    topics: std::env::var("KAFKA_TOPICS")
      .unwrap()
      .split(",")
      .map(|s| s.to_string())
      .collect::<Vec<String>>(),
  });

  let app = App::new(Arc::new(stream_processor) as Arc<dyn StreamProcessor + Send + Sync>);

  app.commands.payments.listen_for_messages.execute().await;
}
