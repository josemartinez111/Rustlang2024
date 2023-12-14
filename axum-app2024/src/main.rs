// ticket_example
// ________________________________________________________

use axum_app2024::web::start_app::start_app;
use utility_lib::axum::response::Response;
use utility_lib::utils::utilities::{debug_print, format_print};
// ________________________________________________________

#[allow(dead_code)]
async fn main_response_mapper(response: Response) -> Response {
  debug_print(
    "main_response_mapper".to_owned(),
    "RES_MAPPER".to_owned(),
    "".to_owned(),
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















