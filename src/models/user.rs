use chrono::NaiveDateTime;
use entity::user::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserModel {
    #[serde(skip_serializing)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUserResponseModel {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserModel {
    pub name: String,
}

impl From<Model> for UserModel {
    fn from(value: Model) -> Self {
        UserModel {
            id: value.id,
            name: value.name,
            email: value.email,
            password: value.password,
            balance: value.balance,
            created_at: value.created_at,
        }
    }
}
