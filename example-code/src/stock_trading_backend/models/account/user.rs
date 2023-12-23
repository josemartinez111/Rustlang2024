// FILE: models/user.rs
// ___________________________________________________________

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// _______________________________________________

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  // `id` `Arc<String>` to allow efficient sharing
  // Use `serde` attribute to specify custom deserialization
  pub id: String,
  // Assuming ID is an integer
  pub username: String,
  pub pwd_hash: String,
  pub email: String,
  pub token: String, // Token will be a String
}
// ___________________________________________________________

impl User {
  #[allow(dead_code)]
  pub fn new<Str: Into<String>>(username: Str, pwd_hash: Str, email: Str, token: Str) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      username: username.into(),
      pwd_hash: pwd_hash.into(),
      email: email.into(),
      token: token.into(),
    }
  }
}
// _______________________________________________
