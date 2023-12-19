// main.rs
// ________________________________________________________

use std::io::Error;

use utility_lib::utils::utilities::{code_spacer, format_print, to_json_with_prefix, Void};

use crate::models::account::user::User;
use crate::models::trading::order::{Order, OrderStatus};
use crate::models::trading::stock::Stock;
use crate::models::trading::trade_account::TradeAccount;

mod models;
// __________________________________________________

fn main() -> Result<Void, Error> {
  format_print("_", 45);
  // __________________________________________________

  let mock_user = User::new(
    1,
    "Jesse",
    "some password",
    "mockemail@gmail.com",
    "12638ddnis",
  );

  to_json_with_prefix("Testing User object", &mock_user)?;
  code_spacer(); // *******************************************************

  let mock_trade_acc = TradeAccount::new(1234567);
  to_json_with_prefix("Testing Trade Account object", &mock_trade_acc)?;
  code_spacer(); // *******************************************************

  let stocks = vec![
    Stock::new("AAPL", "Apple Inc.", 150.00),
    Stock::new("MSFT", "Microsoft Corporation", 250.00),
    Stock::new("GOOGL", "Alphabet Inc.", 2700.00),
    Stock::new("AMZN", "Amazon.com, Inc.", 3300.00),
  ];

  stocks.iter().for_each(|stock: &Stock| {
    match to_json_with_prefix("Stock", stock) {
      Ok(_) => {} // the function itself prints the result
      Err(err) => eprintln!("Error converting stock to json: {}", err)
    }
  });
  code_spacer(); // *******************************************************

  // Start with a Buy order
  let order_type = OrderStatus::Buy;

  stocks.iter().for_each(|stock: &Stock| {
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

    let price_per_share: &f64 = &stock.current_price;

    // Create a new order with the specified details
    let mut order = Order::new(
      1,
      stock.clone(), // Stock impl the `Clone` trait
      quantity,
      price_per_share.to_owned(),
    );

    format_print("=", 33);
    println!("Before completion:");

    // should display a msg stating no orders made
    order.display_details();
    // Now complete the order to display the results
    order.complete_order();

    println!("\nAfter completion:");
    order.display_details();
  });
  code_spacer(); // *******************************************************
  // __________________________________________________
  format_print("_", 45);
  Ok(())
}
// __________________________________________________________










