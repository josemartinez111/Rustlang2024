// main.rs
// ________________________________________________________

use std::cell::RefCell;
use std::io::Error;
use std::rc::Rc;

use utility_lib::utils::utilities::{
  code_spacer, color_format,
  format_print,
  to_json_with_prefix,
  Void,
};

use crate::models::account::account_status::AccountStatus;
use crate::models::account::user::User;
use crate::models::trading::order::{Order, OrderStatus};
use crate::models::trading::stock::Stock;
use crate::models::trading::trade_account::TradeAccount;
use crate::utils::helpers::run_stock_order_example;

mod models;
mod utils;
// __________________________________________________

fn main() -> Result<Void, Error> {
  format_print("_", 45);
  // __________________________________________________

  let mock_trade_user = User::new(
    "Jesse",
    "some password",
    "mockemail@gmail.com",
    "12638ddnis",
  );

  to_json_with_prefix("Testing User object", &mock_trade_user)?;
  code_spacer(); // *******************************************************

  let mock_trade_acc = TradeAccount::new(
    Rc::new(mock_trade_user),
    &Rc::new(65000.00),
  );

  color_format("Testing Trade Account object", mock_trade_acc);
  code_spacer(); // *******************************************************

  let stocks = vec![
    Stock::new("AAPL", "Apple Inc.", 150.00),
    Stock::new("MSFT", "Microsoft Corporation", 250.00),
    Stock::new("GOOGL", "Alphabet Inc.", 2700.00),
    Stock::new("AMZN", "Amazon.com, Inc.", 3300.00),
  ];

  stocks.iter().for_each(|stock| {
    match stock.to_json_with_formatted_price() {
      Ok(stock) => {
        color_format("Stock", stock);
      }
      Err(err) => eprintln!("Error converting stock to json: {}", err)
    }
  });
  code_spacer(); // *******************************************************

  // Start with a Buy order
  let order_type = OrderStatus::Buy;
  run_stock_order_example(&stocks, order_type);
  code_spacer(); // *******************************************************

  // Created a new User
  let acc_status_user = Rc::new(User::new(
    "Jesse",
    "getit2345",
    "mockemail@gmail.com",
    "12638ddnis",
  ));

  // Clone the Rc<User> before passing it to TradeAccount::new
  let acc_status_user_clone = Rc::clone(&acc_status_user);

  let acc_status_user_acc = TradeAccount::new(
    acc_status_user_clone,
    &Rc::new(75000.00),
  );

  // Use Rc::clone to safely access User's id without moving the original Rc<User>
  let user_id_rc = acc_status_user.id.clone();

  println!("Mock User ID: {:?}", user_id_rc);


  // Create an order
  let apple_stock_refcell = &Rc::new(RefCell::new(stocks[0].clone()));

  let last_order = Order::new(
    &acc_status_user,
    apple_stock_refcell,
    10,
    apple_stock_refcell.borrow().current_price,
  );

  // Create an AccountStatus instance for the users
  let account_status = AccountStatus::new(
    &acc_status_user.clone(),
    &Rc::new(acc_status_user_acc),
    &Rc::new(last_order),
  );

  color_format("Testing Account Status", account_status.unwrap());
  code_spacer(); // *******************************************************
  // __________________________________________________
  format_print("_", 45);
  Ok(())
}
// __________________________________________________________










