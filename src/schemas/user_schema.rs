use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserBodySchema {
    pub email: String,
    pub name: String,
    pub password: String,
}

pub struct CreateUserSchema {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
}
