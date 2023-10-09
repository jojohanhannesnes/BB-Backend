use chrono::NaiveDateTime;
use sea_orm::DeriveActiveModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub balance: i64,
    pub created_at: NaiveDateTime,
}
#[derive(Serialize, Deserialize)]
pub struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserModel {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserResponseModel {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserModel {
    pub name: String,
}
