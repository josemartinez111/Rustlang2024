// FILE: trading/stock.rs
// ___________________________________________________________

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utility_lib::iso_currency::Currency;
use utility_lib::utils::utilities::format_price_with_currency_str;
// ___________________________________________________________

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
  pub symbol: String,
  pub name: String,
  pub current_price: f64,
  pub quantity_owned: i32,
}
// ___________________________________________________________

impl Stock {
  pub fn new<Str: Into<String>>(symbol: Str, name: Str, current_price: f64) -> Self {
    Self {
      symbol: symbol.into(),
      name: name.into(),
      current_price,
      quantity_owned: 0, // Initialize the quantity owned to 0
    }
  }

  pub fn to_json_with_formatted_price(&self) -> serde_json::Result<String> {
    //   pub symbol: String,
    //   pub name: String,
    //   pub current_price: f64,
    //   pub quantity_owned: i32,

    let formatted_price = format_price_with_currency_str(
      &Currency::USD,
      format!("{:.2}", self.current_price)
    );

    let stock_with_formatted_price = json!({
            "symbol": self.symbol,
            "name": self.name,
            "current_price": formatted_price,
            "quantity_owned": self.quantity_owned
        });

    serde_json::to_string_pretty(&stock_with_formatted_price)
  }
}
// _______________________________________________

impl Display for Stock {
  fn fmt(&self, format_stock: &mut Formatter<'_>) -> std::fmt::Result {
    //   pub symbol: String,
    //   pub name: String,
    //   pub current_price: f64,
    //   pub quantity_owned: i32,
    let formatted_currency_price = format_price_with_currency_str(
      &Currency::USD,
      format!("{:.2}", self.current_price)
    );

    write!(
      format_stock,
      "\nStock Symbol: ( {} )\
      \nName: ( {} )\
      \nCurrent Stock Price: {}\
      \nQuantity Owned: ( {} )",
      self.symbol,
      self.name,
      formatted_currency_price,
      self.quantity_owned,
    )
  }
}
// _______________________________________________














