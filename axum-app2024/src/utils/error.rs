// FILE: utils/errors.rs
// ___________________________________________________________

use axum::http::StatusCode;
use axum::response::IntoResponse;

pub type Result<T> = core::result::Result<T, Error>;
// ___________________________________________________________

#[derive(Debug)]
pub enum Error {
  InvalidCredentials,
}
// ___________________________________________________________

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response {
    println!("{:<12} - Error - {self:?}", "INTO_RESPONSE");

    // Temporary ERROR
    let tuple_error_response = (
      StatusCode::INTERNAL_SERVER_ERROR,
      "UNHANDLED_CLIENT_ERROR"
    );

    tuple_error_response.into_response()
  }
}
// ___________________________________________________________