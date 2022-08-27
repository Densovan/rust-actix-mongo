use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub bio: String,
}
