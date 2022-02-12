use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Deserialize;
use tracing::instrument;
use tracing::{debug, info};
use tracing_actix_web::TracingLogger;

#[derive(Debug, Deserialize)]
struct CreateOrderInput {
  product_id: String,
  user_id: String,
  quantity: i64,
}

#[post("/orders")]
#[instrument]
async fn create_order(order: Json<CreateOrderInput>) -> impl Responder {
  info!("{:?}", order);
  HttpResponse::Created()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var(
    "RUST_LOG",
    std::env::var("RUST_LOG").unwrap_or_else(|_| format!("{}=trace", env!("CARGO_PKG_NAME"))),
  );

  dotenv().ok();

  let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());

  tracing_subscriber::fmt()
    .with_writer(move || non_blocking_writer.clone())
    .init();

  let host = std::env::var("HOST").unwrap();
  let port = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

  HttpServer::new(move || {
    App::new()
      .wrap(TracingLogger)
      // NOTE: we should only accept requests that come from our frontend.
      .wrap(Cors::permissive())
      .service(create_order)
  })
  .bind((host, port))?
  .run()
  .await
}
