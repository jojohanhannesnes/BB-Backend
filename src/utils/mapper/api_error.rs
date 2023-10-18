use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use super::HEADER;

#[derive(Serialize, Deserialize, Debug)]
pub enum AppError {
    AuthTokenNotFound,
    AuthTokenError,
    UserNotFound,
    UserAlreadyExist,
    DbError,
    InvalidPassword,
    PasswordVerifyError,
    PasswordEncryptionError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIError {
    #[serde(skip_serializing)]
    pub status: AppError,
    pub message: String,
}

impl APIError {
    pub fn new(status: AppError, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status,
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let (status_code, error_code) = match self.status {
            AppError::AuthTokenNotFound => (StatusCode::UNAUTHORIZED, 21),
            AppError::AuthTokenError => (StatusCode::UNAUTHORIZED, 22),
            AppError::UserNotFound => (StatusCode::BAD_REQUEST, 10),
            AppError::UserAlreadyExist => (StatusCode::CONFLICT, 11),
            AppError::DbError => (StatusCode::BAD_REQUEST, 31),
            AppError::InvalidPassword => (StatusCode::UNAUTHORIZED, 41),
            AppError::PasswordVerifyError => (StatusCode::UNAUTHORIZED, 42),
            AppError::PasswordEncryptionError => (StatusCode::UNAUTHORIZED, 43),
        };

        let response = APIErrorResponse {
            message: self.message,
            error_code,
        };

        (status_code, HEADER, Json(response)).into_response()
    }
}

#[derive(Serialize, Deserialize)]
struct APIErrorResponse {
    message: String,
    error_code: i32,
}
