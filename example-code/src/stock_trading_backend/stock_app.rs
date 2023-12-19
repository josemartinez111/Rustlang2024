// main.rs
// ________________________________________________________

use std::io::Error;
use utility_lib::utils::utilities::{format_print, to_json_with_prefix, Void};
use crate::models::trade_account::TradeAccount;

use crate::models::user::User;

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
  format_print("*", 45);
  println!();

  let mock_trade_acc = TradeAccount::new(1234567);
  to_json_with_prefix("Testing Trade Account object", &mock_trade_acc)?;
  // __________________________________________________
  format_print("_", 45);
  Ok(())
}
// __________________________________________________________