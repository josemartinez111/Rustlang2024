// FILE: tests/quick_dev.rs
// ___________________________________________________________

use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use httpc_test::{Client, Response};
use serde_json::json;

// ___________________________________________________________

#[tokio::test]
async fn quick_dev() -> Result<()> {
  // Have to parse the date header in GMT (Since itself the default & cant be changed)
  let http_client: Client = httpc_test::new_client("http://localhost:8080")?;
  let response: Response = http_client.do_get("/health_check?name=OK").await?;
  let response2: Response = http_client.do_get("/health_check2/Alias111").await?;
  // let main_response: Response = http_client.do_get("/src/ticket_example").await?;

  // Testing login
  let req_login = http_client.do_post(
    "/api/login",
    json!({
      "username": "admin",
      "pwd": "alias111"
    })
  );

  // Parse and print the date header in EST
  if let Some(date_header) = response.header("date") {
    let date_utc = DateTime::parse_from_rfc2822(&date_header)?;
    println!("DateTime in GMT (original): {}", date_utc);

    let est_offset_hours = -5;
    let seconds_per_hour = 3600;
    let est_offset = FixedOffset::east_opt(est_offset_hours * seconds_per_hour)
      .expect("Invalid time zone offset");

    let date_est: DateTime<FixedOffset> = date_utc.with_timezone(&est_offset);
    println!("DateTime in EST (converted): {}", date_est);
  }

  // Print response with colored output
  response.print().await?;
  response2.print().await?;
  // main_response.print().await?;
  req_login.await?.print().await?;
  Ok(())
}
// ___________________________________________________________