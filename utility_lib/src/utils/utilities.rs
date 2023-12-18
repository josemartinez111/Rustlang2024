// FILE: utils/utilities.rs
// ____________________________________________________

use std::fmt::Display;
use std::io::Error;

use color_print::cprintln;
use serde::Serialize;
use yansi::{Color, Paint, Style};
use yansi::Color::{Black, Blue, Magenta, Yellow};

// _______________________________________________

pub type Void = ();
// ____________________________________________________

pub fn format_print(arg: &str, num: usize) -> Void {
  cprintln!("<bold, b!>{}</bold, b!>\n", arg.repeat(num));
}
// ____________________________________________________

pub fn color_format<Type: Display>(prefix: &str, data: Type) {
  cprintln!(
        "<bold, w!><u>{}</u></bold, w!>: <bold, b>{}</bold, b>",
        prefix,
        data
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













