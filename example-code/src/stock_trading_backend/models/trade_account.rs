// FILE: models/trade_account.rs
// ___________________________________________________________

use uuid::Uuid;

use utility_lib::rand;
use utility_lib::rust_decimal::Decimal;

// ___________________________________________________________

pub struct TradeAccount {
  pub account_id: String,
  pub user_id: i32,
  pub balance: Decimal,

}
// ___________________________________________________________

impl TradeAccount {
  pub fn new(user_id: i32) -> Self {
    TradeAccount {
      account_id: Uuid::new_v4().to_string(),
      user_id,
      balance: Decimal::new(rand::random(), 2),
    }
  }
}
// ___________________________________________________________