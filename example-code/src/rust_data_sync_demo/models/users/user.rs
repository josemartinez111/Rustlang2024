// FILE: models/users/users.rs
// ___________________________________________________________

use uuid::Uuid;
use example_code::utils::rc_move::ArcMove;

#[derive(Debug)]
pub struct User {
  data: ArcMove<UserData>, // Use ArcMove for thread-safe internal state
}
// ___________________________________________________________

#[derive(Debug)]
#[allow(dead_code)]
struct UserData {
  id: Uuid,
  username: String,
  password: String,
  is_logged_in: bool,
}

// ___________________________________________________________

impl User {
  // Create a new User instance
  pub fn new(username: String, password: String) -> Self {
    let user = UserData {
      id: Uuid::new_v4(),
      username,
      password,
      is_logged_in: false,
    };

    User { data: ArcMove::new(user) }
  }

  // Read-only access to ID
  pub fn get_id(&self) -> Result<Uuid, String> {
    self.data.read_state()
      .map(|data| data.id)
      .map_err(|e| format!("Failed to read ID: {:?}", e))
  }

  // Read-only access to username
  pub fn get_username(&self) -> Result<String, String> {
    self.data.read_state()
      .map(|data| data.username.clone())
      .map_err(|e| format!("Failed to read username: {:?}", e))
  }

  // Read-only access to login status
  pub fn is_logged_in(&self) -> Result<bool, String> {
    self.data.read_state()
      .map(|data| data.is_logged_in)
      .map_err(|e| format!("Failed to read login status: {:?}", e))
  }

  // Update login status
  pub fn set_login_status(&self, status: bool) -> Result<(), String> {
    self.data.update_state()
      .map(|mut data| data.is_logged_in = status)
      .map_err(|e| format!("Failed to update login status: {:?}", e))
  }

  // Validate and update password
  pub fn update_password(&self, old_password: &str, new_password: String) -> Result<(), String> {
    self.data.update_state()
      .map_err(|e| format!("Failed to update password: {:?}", e))
      .and_then(|mut data| {
        if data.password != old_password {
          return Err("Incorrect password".to_string());
        }

        data.password = new_password;
        Ok(())
      })
  }


  // Authenticate user
  pub fn authenticate(&self, password: &str) -> Result<(), String> {
    let data = self.data.read_state()
      .map_err(|e| format!("Failed to authenticate: {:?}", e))?;

    if data.password != password {
      return Err("Authentication failed".to_string());
    }
    Ok(())
  }
}
// ___________________________________________________________