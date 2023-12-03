// FILE: utils/utilities.rs
// ____________________________________________________

use yansi::{Color, Paint, Style};
use yansi::Color::{Black};

pub type Void = ();
// ____________________________________________________

pub fn format_print(arg: &str, num: usize) -> Void {
  println!("{}\n", arg.repeat(num));
}
// ____________________________________________________

pub fn bg_paint(bg_color: Color, str_arg: String) -> Paint<String> {
  let style = Style::new(Black).bold().bg(bg_color).underline();
  Paint::new(str_arg).with_style(style)
}
// ____________________________________________________
