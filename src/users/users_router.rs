use axum::extract::State;
use axum::response::{IntoResponse, Json};
use axum::routing::{get, post, Router};
use serde::Deserialize;

use super::users_model::{SharedUsersState, UsersCollection};

pub async fn get_all_users(State(state): State<SharedUsersState>) -> impl IntoResponse {
  let users = state.lock().await.try_get_all_users().await.unwrap();
  Json(users)
}
