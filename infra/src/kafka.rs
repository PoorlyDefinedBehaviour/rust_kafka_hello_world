use std::time::Duration;

use async_trait::async_trait;
use contracts::stream_processor;
use rdkafka::{
  config::RDKafkaLogLevel,
  consumer::{Consumer, StreamConsumer},
  producer::{FutureProducer, FutureRecord},
  util::Timeout,
  ClientConfig, Message,
};
use tracing::{info, instrument};

#[derive(Debug)]
pub struct Config {
  pub group_id: String,
  pub bootstrap_servers: String,
  pub enable_partition_eof: bool,
  pub session_timeout_ms: usize,
  pub message_timeout_ms: usize,
  pub enable_auto_commit: bool,
  pub topics: Vec<String>,
}

pub struct Kafka {
  config: Config,
  consumer: StreamConsumer,
  producer: FutureProducer,
}

impl Kafka {
  #[instrument]
  pub fn new(config: Config) -> Self {
    let consumer: StreamConsumer = ClientConfig::new()
      .set("group.id", &config.group_id)
      .set("bootstrap.servers", &config.bootstrap_servers)
      .set(
        "enable.partition.eof",
        config.enable_partition_eof.to_string(),
      )
      .set("session.timeout.ms", config.session_timeout_ms.to_string())
      .set("enable.auto.commit", config.enable_auto_commit.to_string())
      .set_log_level(RDKafkaLogLevel::Debug)
      .create()
      .expect("unable to create kafka consumer");

    let topics: Vec<&str> = config.topics.iter().map(|s| s.as_ref()).collect();

    consumer.subscribe(&topics).expect(&format!(
      "unable to subscribe to topics. topics={:?}",
      &config.topics
    ));

    let producer: FutureProducer = ClientConfig::new()
      .set("bootstrap.servers", &config.bootstrap_servers)
      .set("message.timeout.ms", "5000")
      .create()
      .expect("unable to create kafka producer");

    Self {
      config,
      consumer,
      producer,
    }
  }
}

#[async_trait]
impl stream_processor::StreamProcessor for Kafka {
  #[instrument(skip(self))]
  async fn recv(&self) -> Result<stream_processor::Message, stream_processor::ReceiveError> {
    match self.consumer.recv().await {
      Ok(message) => Ok(stream_processor::Message {
        key: message
          .key()
          .map(|buffer| String::from_utf8_lossy(buffer).to_string()),
        partition: message.partition(),
        topic: message.topic().to_string(),
        offset: message.offset(),
        timestamp: message.timestamp().to_millis(),
      }),
      Err(e) => Err(format!("{}", e)),
    }
  }

  #[instrument(skip(self))]
  async fn send(&self, message: stream_processor::SendInput) {
    // TODO: return values returned by rdkafka
    let result = self
      .producer
      .send(
        FutureRecord::to(&message.topic)
          .payload(&message.payload)
          .key(&message.key),
        Timeout::After(Duration::from_secs(0)),
      )
      .await;

    info!("send result={:?}", result);
  }
}
