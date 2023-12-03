// FILE: handlers/health_check.rs
// ___________________________________________________________

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use log::info;
use yansi::Color::{Blue, Red, Yellow};

use crate::models::health_check::HealthCheck;
use crate::utils::utilities::bg_paint;

// ___________________________________________________________

pub async fn health_check(Query(params): Query<HealthCheck>) -> impl IntoResponse {
  let paint_params = bg_paint(Blue, format!("{params:?}"));
  println!(
    "--> {:<12} - health_check - {paint_params}",
    bg_paint(Yellow, "HANDLER".to_owned(),
    )
  );

  let name: &str = params.name.as_deref()
    .unwrap_or_else(|| {
      info!("params.name not found - 404");
      "USING-DEFAULT"
    });

  Html(format!("<h1>Health Check--><strong>{name}</strong></h1>"))
}


pub async fn health_check2(Path(name): Path<String>) -> impl IntoResponse {
  let paint_name = bg_paint(Blue, format!("{name:?}"));
  println!(
    "--> {:<12} - health_check2 - {paint_name}",
    bg_paint(Red, "HANDLER".to_owned(),
    )
  );

  Html(format!("<h1>Health Check #2--><strong>{name}</strong></h1>"))
}
// ___________________________________________________________