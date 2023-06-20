use asserhttp::*;

mod helpers;
use helpers::setup;

#[tokio::test]
async fn health_router_responds_correctly() {
  let address = setup().await;

  reqwest::get(format!("http://{address}/api/health"))
    .await
    .expect_status_ok()
    .expect_body_text_eq("OK");
}
