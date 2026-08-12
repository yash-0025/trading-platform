use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use crate::errors::{TradingError, Result};

#[derive(Debug, Default)]
pub struct UserManager {
    users: HashMap<Uuid, User>,
    username_index: HashMap<String, Uuid>,
}

impl UserManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, username: String, password: &str) -> Result<&User> {
       if self.username_index.contains_key(&username) {
        return Err(TradingError::InvalidQuantity {
            message: format!("Username '{}' already exists", username),
        });
       }

       let user = User::new(username.clone(), password.to_string());
       let user_id = user.id;

       self.users.insert(user_id, user);
       self.username_index.insert(username, user_id);

       Ok(self.users.get(&user_id).unwrap())
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<&User> {
        let user_id = self.username_index.get(username).ok_or_else(|| {
            TradingError::InvalidQuantity {
                message: "Invalid Credentials".into(),
            }
        })?;

        let user = self.users.get(user_id).ok_or_else(|| {
            TradingError::InvalidQuantity {
                message: "User record missing".into(),
            }
        })?;

        if user.verify_password(password) {
            Ok(user) 
        } else {
            Err(TradingError::InvalidQuantity {
                message: "Invalid Credentials".into(),
            })
        }
    }

    
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {

    pub fn new(username: String, password: String) -> Self {
        let id = Uuid::new_v4();
        let password_hash = Self::hash_password(password).to_string();
        let created_at = Utc::now();

        User {
            id,
            username,
            password_hash,
            created_at,
        }
    }

    pub fn hash_password(password: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        Self::hash_password(password.to_string()) == self.password_hash
    }
}

