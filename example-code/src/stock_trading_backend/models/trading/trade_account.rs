// FILE: models/trade_account.rs
// ___________________________________________________________

use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use serde_json::{json, Value};

use uuid::Uuid;

use utility_lib::iso_currency::Currency;
use utility_lib::utils::utilities::format_price_with_currency_str;

use crate::models::account::user::User;

// ___________________________________________________________

#[derive(Debug, Clone)]
pub struct TradeAccount {
  pub user: Rc<User>,
  pub account_id: String,
  pub user_id: String,
  pub balance: f64,
}
// ___________________________________________________________

impl TradeAccount {
  pub fn new(user: Rc<User>, balance: &Rc<f64>) -> Self {
    // Clone the Rc<User> to increase the reference count.
    // This allows the original `users` to be used later without being moved.
    let user_clone = Rc::clone(&user);

    Self {
      user,
      account_id: Uuid::new_v4().to_string(), // generates default UUID
      user_id: user_clone.id.clone(),
      balance: *balance.deref(),
    }
  }
}
// ___________________________________________________________

impl Display for TradeAccount {
  fn fmt(&self, format_trade_account: &mut Formatter<'_>) -> std::fmt::Result {
    // Construct a JSON representation of the User
    let user_json: Value = json!({
            "id": self.user.id,
            "username": self.user.username,
            "email": self.user.email,
            "token": self.user.token
            // Not including pwd_hash for security reasons
        });

    // Convert the JSON value to a pretty-printed string
    let pretty_user = serde_json::to_string_pretty(&user_json)
      .unwrap_or_else(|_| "Failed to serialize users".to_string());

    // Write the non-JSON parts
    write!(
      format_trade_account,
      "\nAccount ID: {}\
            \nUser ID: {}\
            \nBalance: {}\n",
      self.account_id,
      self.user_id,
      format_price_with_currency_str(
        &Currency::USD,
        format!("{:.2}", self.balance),
      ),
    )?;

    // Write the JSON part separately
    writeln!(format_trade_account, "User: {}", pretty_user)
  }
}
// ___________________________________________________________