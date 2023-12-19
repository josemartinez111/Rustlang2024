// FILE: trading/stock.rs
// ___________________________________________________________

use serde::{Deserialize, Serialize};

use utility_lib::iso_currency::Currency;
use utility_lib::utils::utilities::format_price_with_currency_str;
// ___________________________________________________________

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
  pub symbol: String,
  pub name: String,
  pub current_price: f64,
  pub formatted_current_price: String,
}
// ___________________________________________________________

impl Stock {
  pub fn new<Str: Into<String>>(symbol: Str, name: Str, current_price: f64) -> Self {
    Self {
      symbol: symbol.into(),
      name: name.into(),
      current_price,
      formatted_current_price: format_price_with_currency_str(
        &Currency::USD, // using `USD->$` currency symbol
        format!("{:.2}", current_price), // converted `price` from `f64` to a `String`
      ),
    }
  }

  // /// Creates a new `Stock` instance wrapped in an `Arc`
  // /// for thread-safe shared access in web applications.
  // /// Use this for efficient concurrent handling of
  // /// `Stock` data without cloning for each request.
  // ///
  // /// # Arguments
  // /// * `symbol` - The stock symbol.
  // /// * `name` - The company name.
  // /// * `current_price` - The current stock price.
  // ///
  // /// # Returns
  // /// An `Arc<Stock>` instance.
  // pub fn arc_stock<Str: Into<String>>(symbol: Str, name: Str, current_price: f64) -> Arc<Stock> {
  //   Arc::new(Stock::new(symbol, name, current_price))
  // }
}
// ___________________________________________________________












