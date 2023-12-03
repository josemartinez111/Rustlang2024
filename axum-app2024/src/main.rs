// main.rs
// ________________________________________________________

use std::io::Error;

use axum::Router;
use tokio::net::TcpListener;

use axum_app2024::{
  utils::utilities::format_print,
  web::auth::routes_login,
};
use axum_app2024::web::{
  routes::{routers, tower_routes_static},
  run_server::run_server,
};
// ________________________________________________________

#[tokio::main]
async fn main() {
  format_print("_", 45);
  // __________________________________________________
  env_logger::init();
  // Our router
  let all_routes = Router::new()
    .merge(routers())
    .merge(routes_login::routes())
    .fallback_service(tower_routes_static());

  // Start the server
  let listener = TcpListener::bind("127.0.0.1:8080").await
    .unwrap_or_else(|err: Error| {
      eprintln!("Failed to bind to port 8080: {}", err);
      std::process::exit(1);
    });

  run_server(all_routes, listener).await;
  // __________________________________________________
  format_print("_", 45);
}
// __________________________________________________________















