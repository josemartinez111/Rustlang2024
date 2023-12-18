// main.rs
// ________________________________________________________

use std::io::Error;
use utility_lib::utils::utilities::{format_print, to_json_with_prefix, Void};

use crate::models::user::User;

mod models;
// __________________________________________________


fn main() -> Result<Void, Error> {
  format_print("_", 45);
  // __________________________________________________

  let mock_user = User::new(
    1,
    "Jesse",
    "somewhere password",
    "mockemail@gmail.com",
    "12638ddnis",
  );

  to_json_with_prefix("Testing User object", &mock_user)?;
  // __________________________________________________
  format_print("_", 45);
  Ok(())
}
// __________________________________________________________