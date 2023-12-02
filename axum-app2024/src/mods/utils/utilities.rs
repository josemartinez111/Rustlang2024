// FILE: utils/utilities.rs
// ____________________________________________________

pub type Void = ();
// ____________________________________________________

pub fn format_print(arg: &str, num: usize) -> Void {
  println!("{}\n", arg.repeat(num));
}
// ____________________________________________________
