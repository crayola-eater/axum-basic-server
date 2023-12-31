use asserhttp::*;

mod helpers;
use helpers::setup;

#[tokio::test]
async fn health_router_responds_correctly() {
  let address = setup().await;

  reqwest::get(format!("http://{address}/api/health"))
    .await
    .expect_status_no_content()
    .expect_body_absent();
}
