// FILE: utils/helpers.rs
// ___________________________________________________________

use std::fmt::Display;
use std::io::Error;

use color_print::cprintln;
use iso_currency::Currency;
use serde_json::json;
// ___________________________________________________________

fn color_format<Type: Display>(prefix: &str, data: Type) {
  cprintln!(
        "<bold, white!><underline>{}</underline></bold, white!>: \
         <bold, blue>{}</bold, blue>",
        prefix,
        data
    );
}

pub fn ticket_to_json(ticket_type: &str, concert_name: &str, buyer_name: &str, price: &str) -> Result<(), Error> {
  let res_data = json!({
        ticket_type: {
            "Concert-Name": concert_name,
            "Buyer": buyer_name,
            "Price": format!("{}.00", price)
        }
    });

  match serde_json::to_string_pretty(&res_data) {
    Ok(ticket_data) => {
      color_format("Listing offers Ticket TypesPurchased", ticket_data);
      Ok(())
    }
    Err(err) => {
      eprintln!("Error converting ticket to JSON: {}", err);
      // `into()`: Calls U::from(self).
      // this converts the error from
      // `serde_json::error::Error` to `use std::io::Error;`
      Err(err.into())
    }
  }
}
// ___________________________________________________________

// Define a function to format the price with currency symbol
fn format_price_with_currency(currency: &Currency, price: f64) -> String {
  let currency_symbol = currency.symbol().to_string();
  format!("{}{}", currency_symbol, price)
}
// _______________________________________________

pub fn output_result(concert_name: &str, buyer_name: &str, price: &f64, currency: &Currency) -> Result<(), Error> {
  // Dereference price
  let effective_price = if price == &0.0 { 50.0 } else { *price };
  let formatted_price = format_price_with_currency(currency, effective_price);
  ticket_to_json("StandardTicket", concert_name, buyer_name, &formatted_price)
}

// ___________________________________________________________