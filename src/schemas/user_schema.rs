use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserSchema {
    pub email: String,
    pub name: String,
    pub password: String,
}
