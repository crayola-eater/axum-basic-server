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

#[tokio::test]
async fn create_five_users_and_get_all_of_them() {
  let address = setup().await;

  let futures = ["Alice", "Barbara", "Catherine", "Diana", "Elise"]
    .into_iter()
    .enumerate()
    .map(|(id, username)| async move {
      Client::new()
        .post(format!("http://{address}/api/users"))
        .json(&json!({ "username": username }))
        .send()
        .await
        .expect_status_ok()
        .expect_content_type_json()
        .expect_body_json_eq(json!({ "id": id, "username": username }));
    });

  future::join_all(futures).await;

  reqwest::get(format!("http://{address}/api/users"))
    .await
    .expect_status_ok()
    .expect_content_type_json()
    .expect_body_json_eq(json!([
      { "id": 0, "username": "Alice" },
      { "id": 1, "username": "Barbara" },
      { "id": 2, "username": "Catherine" },
      { "id": 3, "username": "Diana" },
      { "id": 4, "username": "Elise" },
    ]));
}

#[tokio::test]
async fn create_five_users_and_get_each_one_by_id() {
  let address = setup().await;

  let futures = ["Alice", "Barbara", "Catherine", "Diana", "Elise"]
    .into_iter()
    .enumerate()
    .map(|(id, username)| async move {
      Client::new()
        .post(format!("http://{address}/api/users"))
        .json(&json!({ "username": username }))
        .send()
        .await
        .expect_status_ok()
        .expect_content_type_json()
        .expect_body_json_eq(json!({ "id": id, "username": username }));

      reqwest::get(format!("http://{address}/api/users/{id}"))
        .await
        .expect_status_ok()
        .expect_content_type_json()
        .expect_body_json_eq(json!({ "id": id, "username": username }));
    });

  future::join_all(futures).await;
}
