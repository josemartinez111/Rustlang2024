// FILE: models/health_check.rs
// ___________________________________________________________

use serde::Deserialize;
// ___________________________________________________________

#[derive(Debug, Deserialize)]
pub struct HealthCheck {
  pub name: Option<String>,
}
// ___________________________________________________________