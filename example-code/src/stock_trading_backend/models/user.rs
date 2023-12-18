// FILE: models/user.rs
// ___________________________________________________________

use serde::{Deserialize, Serialize};

// _______________________________________________

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  // Assuming ID is an integer
  pub username: String,
  pub pwd_hash: String,
  pub email: String,
  pub token: String, // Token will be a String
}
// ___________________________________________________________

impl User {
  pub fn new<Str: Into<String>>(id: i32, username: Str, pwd_hash: Str, email: Str, token: Str) -> Self {
    Self {
      id,
      username: username.into(),
      pwd_hash: pwd_hash.into(),
      email: email.into(),
      token: token.into(),
    }
  }
}
// ___________________________________________________________