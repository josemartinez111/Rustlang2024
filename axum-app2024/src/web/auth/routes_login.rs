// FILE: web/routes_logins.rs
// ___________________________________________________________

use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use yansi::Color::Blue;

use crate::utils::error::{Error, Result};
use crate::utils::utilities::bg_paint;
// ___________________________________________________________

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
  username: String,
  pwd: String,
}
// ___________________________________________________________

pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}
// ___________________________________________________________

pub async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
  println!("--> {:<12} - api_login", bg_paint(Blue, "HANDLER".to_string()));

  // Convert `username` and `password` from String to &str (slice reference)
  // and match. This allows for efficient comparison without taking ownership.
  match (&payload.username[..], &payload.pwd[..]) {
    // When credentials match expected values, create a success response
    ("admin", "alias111") => {
      // Using serde-json
      let res_body = json!({
        "result":{
          "status": "success"
        }});

      // axum::Json
      let json_success_ok = Json(res_body);
      // Return the response
      Ok(json_success_ok)
    }
    // When credentials don't match, return an error
    _ => Err(Error::InvalidCredentials),
  }
}
// ___________________________________________________________