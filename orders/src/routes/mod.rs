use actix_web::web;
pub mod orders;

pub fn init(config: &mut web::ServiceConfig) {
  orders::init(config);
}
