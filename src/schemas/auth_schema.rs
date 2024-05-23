use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateUserSchema {
    pub email: String,
    pub password: String,
}
