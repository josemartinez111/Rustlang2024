// FILE: trading/order.rs
// ___________________________________________________________


use serde::{Deserialize, Serialize};
use uuid::Uuid;

use utility_lib::iso_currency::Currency;
use utility_lib::utils::utilities::{color_format, format_price_with_currency_str};

use crate::models::trading::stock::Stock;

// ___________________________________________________________

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
  pub order_id: String,
  pub user_id: i32,
  pub stock: Stock,
  pub quantity: i32,
  pub price: f64,
  pub formatted_price: String, // Store the formatted price for display
  pub status: OrderStatus,
}
// ___________________________________________________________

/// `OrderStatus`: Represents the status of a trading order.
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderStatus {
  Buy,
  Sell,
  /// `Pending`: The order has been created but is not yet executed.
  /// It's waiting for certain conditions to be met or for further action.
  Pending,

  /// `Completed`: The order has been fully executed. This
  /// status indicates that the transaction has been completed.
  Completed,
}
// ___________________________________________________________

impl Order {
  pub fn new(user_id: i32, stock: Stock, quantity: i32, price: f64) -> Self {
    Self {
      order_id: Uuid::new_v4().to_string(),
      user_id,
      stock,
      quantity,
      price,
      formatted_price: format_price_with_currency_str(
        &Currency::USD,
        format!("{:.2}", price),
      ),
      status: OrderStatus::Pending,
    }
  }

  #[allow(dead_code)]
  /// `complete_order`: Method to complete an order
  pub fn complete_order(&mut self) {
    self.status = OrderStatus::Completed;
  }

  /// `display_details`: Method to display the order details based on the order status
  pub fn display_details(&self) {
    let total_cost: f64 = self.quantity as f64 * self.price;

    let formatted_total_cost = format_price_with_currency_str(
      &Currency::USD,
      format!("{:.2}", total_cost),
    );

    let order_details_result = format!(
      "Order ID: {}\
      \nUser ID: {}\
      \nStock: {} ({})\
      \nQuantity: {}\
      \nPrice per share: {}\
      \nTotal cost: {}\
      \nStatus: {:?}",
      self.order_id,
      self.user_id,
      self.stock.name,
      self.stock.symbol,
      self.quantity,
      self.price,
      formatted_total_cost,
      self.status
    );

    // custom color_format function
    color_format("Stock Order Details\n", order_details_result);
  }
}
// ___________________________________________________________