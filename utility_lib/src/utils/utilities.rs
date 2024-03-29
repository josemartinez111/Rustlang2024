// FILE: utils/utilities.rs
// ____________________________________________________

use std::fmt::{Display};
use std::io::Error;

use chrono::{DateTime, Local};
use color_print::cprintln;
use iso_currency::Currency;
use rust_decimal::Decimal;
use serde::Serialize;
use yansi::{Color, Paint, Style};
use yansi::Color::{Black, Blue, Magenta, Yellow};

// _______________________________________________

pub type Void = ();
// ____________________________________________________

pub fn format_print(arg: &str, num: usize) -> Void {
  cprintln!("<bold, b!>{}</bold, b!>\n", arg.repeat(num));
}

pub fn code_spacer() {
  format_print("*", 45);
  println!();
}
// ____________________________________________________

pub fn current_date() -> String {
// Get the current local date and time
  let now = Local::now();
  // Format the date and time as "Sep 12, 2020, 12:34 PM"
  let formatted_date: String = now.format("%b %d, %Y, %I:%M %p").to_string();
  formatted_date
}

pub fn current_date_to_str(date_str: &str) -> Result<DateTime<Local>, String> {
  let parsed_date = DateTime::parse_from_str(date_str, "%b %d, %Y, %I:%M %p")
    .map_err(|_| {
      "Failed to parse date".to_string()
    })?;

  Ok(parsed_date.with_timezone(&Local))
}
// ____________________________________________________

pub fn color_format<Type: Display>(prefix: &str, data: Type) {
  cprintln!(
        "<bold, w!><u>{}</u></bold, w!>: <bold, b>{}</bold, b>",
        prefix,
        data // This will now use the Display implementation
    );
}
// ____________________________________________________

pub fn bg_paint(bg_color: Color, str_arg: String) -> Paint<String> {
  let style = Style::new(Black).bold().bg(bg_color).underline();
  let result = Paint::new(str_arg).with_style(style);
  #[allow(clippy::let_and_return)]
  result
}
// ____________________________________________________

pub fn debug_print(str_args: String, painted_str: String, paint_params: String) {
  let formatted_result = format!(
    "--> {:<12} - {:<12} - {:<5}",
    bg_paint(Magenta, str_args),
    bg_paint(Yellow, painted_str),
    bg_paint(Blue, paint_params),
  );

  println!("{}", formatted_result);
}
// ____________________________________________________

/// `to_json_with_prefix` A generic function that takes a serializable object and a prefix string
pub fn to_json_with_prefix<Type: Serialize>(prefix: &str, data: &Type) -> Result<String, Error> {
  let serialized_data = serde_json::to_string_pretty(data)?;
  color_format(prefix, serialized_data.to_owned());
  Ok(serialized_data)
}
// ____________________________________________________

/// `format_price_with_currency_f64` Define a function to format the price with currency symbol
pub fn format_price_with_currency_f64(currency: &Currency, price: Decimal) -> String {
  let currency_symbol = currency.symbol().to_string();
  format!("{}{}", currency_symbol, price)
}

/// `format_price_with_currency_str` Define a function to format the price with currency symbol
pub fn format_price_with_currency_str(currency: &Currency, price: String) -> String {
  let currency_symbol = currency.symbol().to_string();
  format!("{}{}", currency_symbol, price)
}
// ____________________________________________________













