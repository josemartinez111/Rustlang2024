// FILE: models/trade_account.rs
// ___________________________________________________________

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use utility_lib::iso_currency::Currency;
use utility_lib::utils::utilities::format_price_with_currency_str;
// ___________________________________________________________

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeAccount {
  pub account_id: String,
  pub user_id: i32,
  pub balance: String,
}
// ___________________________________________________________

impl TradeAccount {
  pub fn new(user_id: i32) -> Self {
    TradeAccount {
      account_id: Uuid::new_v4().to_string(), // generates default UUID
      user_id,
      balance: format_price_with_currency_str(
        &Currency::USD,
        format!("{}.00", 6500f64),
      ), // default account balance
    }
  }
}
// ___________________________________________________________