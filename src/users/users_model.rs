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

  pub async fn try_get_all_users(&self) -> Result<Vec<User>, ()> {
    Ok(self.users.clone())
  }

  pub async fn try_get_user_by_id(&self, id: usize) -> Result<&User, ()> {
    self.users.iter().find(|user| user.id == id).ok_or(())
  }

  pub async fn try_delete_user_by_id(&mut self, id: usize) -> Result<User, ()> {
    self
      .users
      .iter()
      .position(|user| user.id == id)
      .ok_or(())
      .map(|position| self.users.swap_remove(position))
  }
}

pub type SharedUsersState = Arc<Mutex<UsersCollection>>;
