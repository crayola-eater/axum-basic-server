use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
  id: usize,
  username: String,
}

#[derive(Debug, Default)]
pub struct UsersCollection {
  user_id: usize,
  users: Vec<User>,
}

impl UsersCollection {
  pub async fn new_shared() -> Arc<Mutex<Self>> {
    Arc::new(Mutex::new(UsersCollection::default()))
  }
}

pub type SharedUsersState = Arc<Mutex<UsersCollection>>;
