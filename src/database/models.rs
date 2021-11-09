use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct User {
    username: String,
    password: String,
    email: String,
}

impl User {
    pub fn new(username: String, password_hash: String, email: String) -> Self {
        User {
            username,
            password: password_hash,
            email,
        }
    }

    pub fn username(&self) -> &String {
        &self.username
    }
    pub fn password(&self) -> &String {
        &self.password
    }
    pub fn email(&self) -> &String {
        &self.email
    }
}
