use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

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

