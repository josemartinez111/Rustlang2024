// FILE: handlers/health_check.rs
// ___________________________________________________________

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use log::info;

use crate::models::health_check::HealthCheck;
use crate::utils::utilities::debug_print;

// ___________________________________________________________

pub async fn health_check(Query(params): Query<HealthCheck>) -> impl IntoResponse {
  let f_params = format!("{params:?}");
  debug_print("HANDLER".to_owned(), "health_check".to_owned(), f_params.to_string());

  let name: &str = params.name.as_deref()
    .unwrap_or_else(|| {
      info!("params.name not found - 404");
      "USING-DEFAULT"
    });

  Html(format!("<h1>Health Check--><strong>{name}</strong></h1>"))
}

// _______________________________________________

pub async fn health_check2(Path(name): Path<String>) -> impl IntoResponse {
  debug_print("HANDLER".to_owned(), "health_check2".to_owned(), name.to_string());
  Html(format!("<h1>Health Check #2--><strong>{name}</strong></h1>"))
}
// ___________________________________________________________