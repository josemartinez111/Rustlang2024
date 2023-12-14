// FILE: utils/utilities.rs
// ____________________________________________________

use yansi::{Color, Paint, Style};
use yansi::Color::{Black, Blue, Magenta, Yellow};
// _______________________________________________

pub type Void = ();
// ____________________________________________________

pub fn format_print(arg: &str, num: usize) -> Void {
  println!("{}\n", arg.repeat(num));
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
