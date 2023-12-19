// FILE: utils/helpers.rs
// ___________________________________________________________

use std::io::Error;

use serde_json::json;

use utility_lib::iso_currency::Currency;
use utility_lib::utils::utilities::{
  format_price_with_currency_f64,
  to_json_with_prefix,
  Void,
};
// ___________________________________________________________

pub fn ticket_to_json(ticket_type: &str, concert_name: &str, buyer_name: &str, price: &str) -> Result<Void, Error> {
  let res_data = json!({
        ticket_type: {
            "Concert-Name": concert_name,
            "Buyer": buyer_name,
            "Price": format!("{}.00", price)
        }
    });

  to_json_with_prefix("Listing offers Ticket TypesPurchased", &res_data)?;
  Ok(())
}
// ___________________________________________________________


// _______________________________________________

pub fn output_result(concert_name: &str, buyer_name: &str, price: &f64, currency: &Currency) -> Result<(), Error> {
  // Dereference price
  let effective_price = if price == &0.0 { 50.0 } else { *price };
  let formatted_price = format_price_with_currency_f64(currency, effective_price);
  ticket_to_json("StandardTicket", concert_name, buyer_name, &formatted_price)
}

// ___________________________________________________________