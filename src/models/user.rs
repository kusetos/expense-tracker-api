use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct User{
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct NewUserData{
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginData{
    pub email: String,
    pub password: String,
}
