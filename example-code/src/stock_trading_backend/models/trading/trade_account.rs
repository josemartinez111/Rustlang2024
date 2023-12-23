// FILE: models/trade_account.rs
// ___________________________________________________________

use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

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
    // This allows the original `user` to be used later without being moved.
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
    write!(
      format_trade_account,
      "\nUser: {:?}\
      \nAccount ID: {}\
      \nUser ID: {}\
      \nBalance: {}\n",
      self.user,
      self.account_id,
      self.user_id,
      format_price_with_currency_str(
        &Currency::USD,
        format!("{:.2}", self.balance),
      ),
    )
  }
}
// ___________________________________________________________