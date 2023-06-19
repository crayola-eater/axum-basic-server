use asserhttp::*;
use futures::future;
use reqwest::Client;
use serde_json::json;

mod helpers;
use helpers::setup;

#[tokio::test]
async fn create_one_user() {
  let address = setup().await;
  let username = "Abe";

  Client::new()
    .post(format!("http://{address}/api/users"))
    .json(&json!({ "username": username }))
    .send()
    .await
    .expect_status_ok()
    .expect_content_type_json()
    .expect_body_json_eq(json!({ "id": 0, "username": "Abe" }));

  reqwest::get(format!("http://{address}/api/users"))
    .await
    .expect_status_ok()
    .expect_content_type_json()
    .expect_body_json_eq(json!([{ "id": 0, "username": "Abe" }]));
}

#[tokio::test]
async fn get_all_users_when_no_users() {
  let address = setup().await;

  reqwest::get(format!("http://{address}/api/users"))
    .await
    .expect_status_ok()
    .expect_content_type_json()
    .expect_body_json_eq(json!([]));
}
