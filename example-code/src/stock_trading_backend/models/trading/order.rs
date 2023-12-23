// FILE: trading/order.rs
// ___________________________________________________________


use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use utility_lib::iso_currency::Currency;
use utility_lib::utils::utilities::{color_format, format_price_with_currency_str};

use crate::models::account::user::User;
use crate::models::trading::stock::Stock;
// ___________________________________________________________

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Order {
  pub order_id: String,
  pub user_id: String,
  pub stock: Stock,
  pub quantity: i32,
  pub price: f64,
  pub formatted_price: String,
  // Store the formatted price for display
  pub status: OrderStatus,
}
// ___________________________________________________________

/// `OrderStatus`: Represents the status of a trading order.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
  pub fn new(user_id_obj: &Rc<User>, stock: &Rc<RefCell<Stock>>, quantity: i32, price: f64) -> Self {
    let stock_data = match stock.try_borrow() {
      Ok(stock_ref) => stock_ref,
      Err(_) => {
        eprintln!("Failed to borrow stock for creating Order");
        std::process::exit(1)
      }
    };

    Self {
      order_id: Uuid::new_v4().to_string(),
      user_id: user_id_obj.id.to_string(),
      stock: stock_data.to_owned(),
      quantity,
      price,
      formatted_price: format_price_with_currency_str(
        &Currency::USD,
        format!("{:.2}", price),
      ),
      status: OrderStatus::Pending,
    }
  }

  /// `complete_order`: Method to complete an order
  pub fn complete_order(&mut self) {
    self.status = OrderStatus::Completed;
    let mut stock_mutex = self.stock.to_owned();
    // Update the quantity owned in the stock
    stock_mutex.quantity_owned += self.quantity;
  }

  /// `display_details`: Method to display the order details based on the order status
  pub fn display_order_details(&self) {
    let total_cost: f64 = self.quantity as f64 * self.price;

    let formatted_total_cost = format_price_with_currency_str(
      &Currency::USD,
      format!("{:.2}", total_cost),
    );

    let stock_mutex = self.stock.to_owned();
    
    let order_details_result = format!(
      "Order ID: {}\
      \nUser ID: {}\
      \nStock: ( {:?} )\
      \nStock Symbol: ( {:?} )\
      \nQuantity: {:?}\
      \nPrice per share: {}\
      \nTotal cost: {}\
      \nStatus: {:?}
      \n\n( Quantity Owned ): {}",
      self.order_id,
      self.user_id,
      stock_mutex,
      stock_mutex.symbol,
      self.quantity,
      self.formatted_price,
      formatted_total_cost,
      self.status,
      stock_mutex.quantity_owned
    );

    // custom color_format function
    color_format("Stock Order Details\n", order_details_result);
  }
}
// ___________________________________________________________
