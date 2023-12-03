// FILE: web/routes.rs
// ___________________________________________________________

use axum::Router;
use axum::routing::{get, get_service};
use tower_http::services::ServeDir;
use super::handlers::health_check::{health_check, health_check2};
// ___________________________________________________________

pub fn routers() -> Router {
  Router::new()
    .route("/health_check", get(health_check))
    .route("/health_check2/:name", get(health_check2))
}
// _______________________________________________

pub fn tower_routes_static() -> Router {
  #[allow(unused_doc_comments)]
  /// `pub struct ServeDir`: Service that serves files from a given
  /// directory and all its subdirectories.
  ///
  /// The Content-Type will be guessed from the file extension.
  ///
  /// An empty response with status 404 Not Found will be returned if:
  /// - The file doesn't exist
  /// - Any segment of the path contains `..`
  /// - Any segment of the path contains a backslash
  /// - On Unix, any segment of the path referenced as a directory is
  /// actually an existing file (`/file.html/something`)
  /// - We don't have the necessary permissions to read the file
  Router::new()
    .nest_service("/", get_service(ServeDir::new("./")))
}
// ___________________________________________________________