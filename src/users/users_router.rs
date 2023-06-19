use axum::extract::State;
use axum::response::{IntoResponse, Json};
use axum::routing::{get, post, Router};
use serde::Deserialize;

use super::users_model::{SharedUsersState, UsersCollection};

pub async fn get_all_users(State(state): State<SharedUsersState>) -> impl IntoResponse {
  let users = state.lock().await.try_get_all_users().await.unwrap();
  Json(users)
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequestBody {
  username: String,
}

pub async fn create_user(
  State(state): State<SharedUsersState>,
  Json(body): Json<CreateUserRequestBody>,
) -> impl IntoResponse {
  let created = state
    .lock()
    .await
    .try_create_user(body.username)
    .await
    .unwrap();

  Json(created)
}

pub async fn create_users_router() -> Router {
  let shared_state = UsersCollection::new_shared().await;

  Router::new()
    .route("/", get(get_all_users))
    .route("/", post(create_user))
    .with_state(shared_state)
}
