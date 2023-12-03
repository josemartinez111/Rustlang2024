// FILE: web/start_app.rs
// ___________________________________________________________

use std::io::Error;

use axum::Router;
use tokio::net::TcpListener;

use crate::web::auth::routes_login;
use crate::web::routes::{routers, tower_routes_static};
use crate::web::run_server::run_server;
// ___________________________________________________________

pub async fn start_app() {
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
}
// ___________________________________________________________