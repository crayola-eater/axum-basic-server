use axum::extract::{Path, State};
use axum::http::StatusCode;
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

pub async fn get_user_by_id(
  Path(id): Path<usize>,
  State(state): State<SharedUsersState>,
) -> impl IntoResponse {
  let guard = state.lock().await;
  let user = guard.try_get_user_by_id(id).await;

  match user {
    Ok(user) => Json(user).into_response(),
    Err(_) => (StatusCode::NOT_FOUND, "user not found").into_response(),
  }
}

pub async fn create_users_router() -> Router {
  let shared_state = UsersCollection::new_shared().await;

  Router::new()
    .route("/", get(get_all_users))
    .route("/", post(create_user))
    .route("/:user_id", get(get_user_by_id))
    .with_state(shared_state)
}
