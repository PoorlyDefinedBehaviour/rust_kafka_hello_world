use crate::domain::orders::commands::create_order;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateOrderInput {
  product_id: String,
  user_id: String,
  quantity: i64,
}

impl Into<create_order::CreateOrderInput> for CreateOrderInput {
  fn into(self) -> create_order::CreateOrderInput {
    create_order::CreateOrderInput {
      product_id: self.product_id,
      user_id: self.user_id,
      quantity: self.quantity,
    }
  }
}
