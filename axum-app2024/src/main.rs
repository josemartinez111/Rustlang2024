// main.rs
// ________________________________________________________

use std::io::Error;

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::{get, get_service};
use log::info;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use mods::utils::utilities::format_print;

pub mod mods;

// ________________________________________________________

#[derive(Debug, Deserialize)]
pub struct HealthCheck {
  pub name: Option<String>,
}

// ________________________________________________________

pub async fn health_check(Query(params): Query<HealthCheck>) -> impl IntoResponse {
  println!("--> {:<12} - health_check - {params:?}", "HANDLER");

  let name = params.name.as_deref()
    .unwrap_or("USING DEFAULT - params.name not found - 404");

  Html(format!("<h1>Health Check--><strong>{name}</strong></h1>"))
}

pub async fn health_check2(Path(name): Path<String>) -> impl IntoResponse {
  println!("--> {:<12} - health_check2 - {name:?}", "HANDLER");
  Html(format!("<h1>Health Check #2--><strong>{name}</strong></h1>"))
}

pub fn routers() -> Router {
  Router::new()
    .route("/health_check", get(health_check))
    .route("/health_check2/:name", get(health_check2))
}

pub fn tower_routes_static() -> Router {
  Router::new().nest_service(
    "/",
    get_service(ServeDir::new("./"))
  )
}
// __________________________________________________

#[tokio::main]
async fn main() {
  format_print("_", 45);
  // __________________________________________________
  env_logger::init();
  // Our router
  let routes = Router::new().merge(routers())
    .fallback_service(tower_routes_static());

  // Start the server
  let listener = TcpListener::bind("127.0.0.1:8080").await
    .unwrap_or_else(|err: Error| {
      eprintln!("Failed to bind to port 8080: {}", err);
      std::process::exit(1);
    });

  // Retrieve the local address and handle any potential error
  let local_addrs = listener.local_addr().unwrap_or_else(|err: Error| {
    eprintln!("Failed to get local address: {}", err);
    std::process::exit(1);
  });

  // Log the listening address
  info!("\n\n\nNOW LISTENING ON - http://{}", local_addrs);

  // Serve the application and handle any error
  if let Err(err) = axum::serve(listener, routes.into_make_service())
    .await {
    eprintln!("Failed to start server: {}", err);
  }
  // __________________________________________________
  format_print("_", 45);
}
// __________________________________________________________















