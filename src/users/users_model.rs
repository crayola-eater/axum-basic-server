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

  pub async fn try_create_user(&mut self, username: String) -> Result<User, ()> {
    let created = User {
      id: self.user_id,
      username,
    };

    self.user_id += 1;
    self.users.push(created.clone());

    Ok(created)
  }
}

pub type SharedUsersState = Arc<Mutex<UsersCollection>>;
