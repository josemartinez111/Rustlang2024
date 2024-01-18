// FILE: models/users.rs
// ___________________________________________________________

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// _______________________________________________

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub username: String,
  pub pwd_hash: String,
  pub email: String,
  pub token: String,
}
// ___________________________________________________________

impl User {
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
