// main.rs
// ________________________________________________________

use axum::response::Response;
use axum_app2024::{
  utils::utilities::format_print,
  web::start_app::start_app,
};
use axum_app2024::utils::utilities::debug_print;
// ________________________________________________________

#[allow(dead_code)]
async fn main_response_mapper(response: Response) -> Response {
  debug_print(
    "main_response_mapper".to_owned(),
    "RES_MAPPER".to_owned(),
    "".to_owned()
  );
  response
}
// ________________________________________________________

#[tokio::main]
async fn main() {
  format_print("_", 45);
  // __________________________________________________

  start_app().await;
  // __________________________________________________
  format_print("_", 45);
}
// __________________________________________________________















