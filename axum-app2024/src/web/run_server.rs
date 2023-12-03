// FILE: web/run_server.rs
// ___________________________________________________________

use std::io::Error;
use axum::Router;
use log::info;
use tokio::net::TcpListener;
// ___________________________________________________________

pub async fn run_server(all_routes: Router, listener: TcpListener) {
// Retrieve the local address and handle any potential error
  let local_addrs = listener.local_addr().unwrap_or_else(|err: Error| {
    eprintln!("Failed to get local address: {}", err);
    std::process::exit(1);
  });

  // Log the listening address
  info!("\n\n\nNOW LISTENING ON - http://{}", local_addrs);

  // Serve the application and handle any error
  if let Err(err) = axum::serve(listener, all_routes.into_make_service())
    .await {
    eprintln!("Failed to start server: {}", err);
  }
}
// ___________________________________________________________