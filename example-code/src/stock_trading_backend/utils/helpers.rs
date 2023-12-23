// FILE: utils/helpers.rs
// ___________________________________________________________

use std::cell::RefCell;
use std::rc::Rc;

use utility_lib::utils::utilities::format_print;

use crate::models::account::user::User;
use crate::models::trading::order::{Order, OrderStatus};
use crate::models::trading::stock::Stock;

// ___________________________________________________________

pub fn run_stock_order_example(stocks: &[Stock], order_type: OrderStatus) {
  stocks.iter().for_each(|stock| {
    // Obtain a lock on the stock to access its properties
    let price_per_share = stock.current_price;

    // Determine the quantity based on the order type
    let quantity = match order_type {
      OrderStatus::Buy => 10,
      OrderStatus::Sell => 5,
      OrderStatus::Pending => {
        println!("Pending orders do not have a specified quantity yet.");
        return; // Skip further processing for this stock
      }
      OrderStatus::Completed => {
        println!("Completed orders do not change quantity.");
        return; // Skip further processing for this stock
      }
    };

    // Creating a `User` to pas to the order
    let user_for_order = Rc::new(User::new(
      "Alias_one11",
      "getit2345",
      "mockemail@gmail.com",
      "12638drty",
    ));

    let cloned_stock: &Rc<RefCell<Stock>> = &Rc::new(RefCell::new(stock.clone()));

    // Create a new order with the specified details
    let mut order = Order::new(
      &user_for_order,
      cloned_stock, // <-- HERE ERROR
      quantity,
      price_per_share,
    );

    format_print("=", 33);
    println!("Before completion:");

    // should display a msg stating no orders made
    order.display_order_details();
    // Now complete the order to display the results
    order.complete_order();

    println!("\nAfter completion:");
    order.display_order_details();
  });
}
// ___________________________________________________________