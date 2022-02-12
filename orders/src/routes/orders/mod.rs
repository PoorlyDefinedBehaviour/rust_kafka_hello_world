use std::sync::Arc;

use actix_web::{
  post,
  web::{self, Data, Json},
  HttpResponse, Responder,
};

use tracing::{info, instrument};

use crate::domain::App;

mod viewmodel;

pub fn init(config: &mut web::ServiceConfig) {
  config.service(create_order);
}

#[post("/orders")]
#[instrument(skip(app))]
async fn create_order(app: Data<App>, order: Json<viewmodel::CreateOrderInput>) -> impl Responder {
  info!("{:?}", order);
  app
    .commands
    .orders
    .create_order
    .execute(order.into_inner().into())
    .await;
  HttpResponse::Created()
}
