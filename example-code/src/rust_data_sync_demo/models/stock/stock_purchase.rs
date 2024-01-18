// FILE: models/stock/stock_purchase.rs
// ___________________________________________________________

use std::sync::RwLockReadGuard;
use uuid::Uuid;

use example_code::utils::rc_move::ArcMove;
use utility_lib::chrono::{DateTime, Local};
use utility_lib::utils::utilities::{current_date, current_date_to_str};

// ___________________________________________________________

#[derive(Debug)]
pub struct StockPurchase {
  // Use ArcMove for thread-safe internal state
  data: ArcMove<StockPurchaseData>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct StockPurchaseData {
  id: Uuid,
  // ID of the associated User
  user_id: Uuid,
  stock_symbol: String,
  shares: u32,
  purchase_price: f64,
  purchase_date: String,
}
// ___________________________________________________________

impl StockPurchase {
  // Create a new StockPurchase instance
  pub fn new(user_id: Uuid, symbol: String, shares: u32, price: f64) -> Self {
    let stock_purchase = StockPurchaseData {
      id: Uuid::new_v4(),
      user_id,
      stock_symbol: symbol,
      shares,
      purchase_price: price,
      purchase_date: current_date(),
    };

    StockPurchase { data: ArcMove::new(stock_purchase) }
  }
  // _______________________________________________

  // Fetches the current market price of the stock
  pub fn current_price(&self) -> Result<f64, String> {
    // Retrieve the stock purchase data from the ArcMove wrapper
    let stock_data: RwLockReadGuard<StockPurchaseData> = self.data.read_state()
      .map_err(|_| "Failed to read stock data".to_string())?;

    // Parse the stored purchase date string into a DateTime<Local> object
    let purchase_date: DateTime<Local> = current_date_to_str(&stock_data.purchase_date)
      .map_err(|_| "Failed to convert purchase date".to_string())?;

    // Calculate the time difference between now and the purchase date
    let current_date_now: DateTime<Local> = Local::now();
    // Calculate the number of days since the stock was purchased
    let days_since_purchase: i64 = current_date_now
      .signed_duration_since(purchase_date)
      .num_days();

    // Apply a model to calculate the current price based on the time elapsed
    // Here, the model is a placeholder for a real price determination logic
    // For each day since purchase, the price changes by a fixed percentage
    // This is a simplification and should be replaced with an actual pricing strategy
    let stock_pct_change: f64 = 1.01;
    let price_change_factor: f64 = stock_pct_change.powi(days_since_purchase as i32);
    let current_price: f64 = stock_data.purchase_price * price_change_factor;

    Ok(current_price)
  }

  // Calculates the profit or loss based on the current market price
  pub fn calculate_profit(&self) -> Result<f64, String> {
    // Fetch the current market price
    let current_price: f64 = self.current_price()?;

    // Access the stock data to get the purchase price and the number of shares
    let stock_data: RwLockReadGuard<StockPurchaseData> = self.data.read_state()
      .map_err(|_| "Failed to read stock data".to_string())?;

    // Calculate the price difference per share
    let price_difference_per_share: f64 = current_price - stock_data.purchase_price;
    // Calculate the total profit or loss
    let total_profit_or_loss: f64 = price_difference_per_share * stock_data.shares as f64;

    Ok(total_profit_or_loss)
  }

  // Validates conditions before selling the stock
  pub fn validate_and_sell_stock(&self, shares_to_sell: u32) -> Result<f64, String> {
    todo!()
  }

  // Returns details of the stock purchase
  pub fn get_purchase_details(&self) -> Result<(String, u32, f64, String), String> {
    todo!()
  }
}
// ___________________________________________________________