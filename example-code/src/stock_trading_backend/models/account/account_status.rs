// FILE: account/account_status.rs
// ___________________________________________________________


use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use std::rc::Rc;

use utility_lib::iso_currency::Currency;
use utility_lib::utils::utilities::{current_date, format_price_with_currency_str, Void};

use crate::models::account::user::User;
use crate::models::trading::order::Order;
use crate::models::trading::stock::Stock;
use crate::models::trading::trade_account::TradeAccount;

// ___________________________________________________________

type Quantity = i32;

#[derive(Debug)]
pub struct AccountStatus {
  // shared across instances of (AccountStatus)
  pub user_id: String,
  pub last_login: String,
  pub balance: f64,
  pub stock_owned: HashMap<Stock, Quantity>,
  pub last_purchase: Rc<Order>,
}
// ___________________________________________________________

impl AccountStatus {
  pub fn new(user_id_obj: &Rc<User>, trade_acc_balance_obj: &Rc<TradeAccount>, last_purchase_order_obj: &Rc<Order>) -> Result<Self, Error> {
    // Validate the users before creating object
    validate_user(user_id_obj, &trade_acc_balance_obj.user_id)?;

    let result = Self {
      user_id: user_id_obj.id.to_string(),
      last_login: current_date(),
      // Dereferences `Rc<f64>` to access the `f64` value inside. Since `f64` implements
      // `Copy`, this operation copies the `f64` value. The `Rc` itself remains unchanged.
      balance: trade_acc_balance_obj.balance,
      stock_owned: HashMap::new(),
      last_purchase: Rc::clone(last_purchase_order_obj),
    };

    Ok(result)
  }
}
// ___________________________________________________________

impl Display for AccountStatus {
  fn fmt(&self, formatted_acc_status: &mut Formatter<'_>) -> std::fmt::Result {
    // pub user_id: Arc<String>,
    //   pub last_login: String,
    //   pub balance: Arc<f64>,
    //   pub stock_owned: HashMap<Stock, Quantity>,
    //   pub last_purchase: Order,
    write!(
      formatted_acc_status,
      "\nUser Id: {}\
      \nLast Login: {}\
      \nBalance: {:#?}\
      \nStock Owned: {:#?}\
      \nLast Purchase: {:#?}\n",
      self.user_id,
      self.last_login,
      format_price_with_currency_str(
        &Currency::USD,
        format!("${:.2}", self.balance),
      ),
      self.stock_owned,
      self.last_purchase
    )
  }
}
// ___________________________________________________________

// Simulates fetching a User by their ID (UUID string)
fn validate_user(user: &Rc<User>, user_id: &str) -> Result<Void, Error> {
  match user.id == user_id {
    true => Ok(()),
    false => {
      Err(Error::new(ErrorKind::NotFound, "User ID does not match"))
    }
  }
}
// ___________________________________________________________







