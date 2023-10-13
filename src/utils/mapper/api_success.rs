use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

const HEADER: [(axum::headers::HeaderName, &str); 1] = [(header::CONTENT_TYPE, "application/json")];

#[derive(Serialize, Deserialize, Debug)]
pub enum AppSuccess {
    UserLoggedIn,
    UserCreated,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APISuccess<T> {
    #[serde(skip_serializing)]
    status: AppSuccess,
    message: String,
    data: T,
}

impl<T> APISuccess<T> {
    pub fn new(status: AppSuccess, message: impl Into<String>, data: T) -> Self {
        APISuccess {
            status,
            message: message.into(),
            data,
        }
    }
}

impl<T> IntoResponse for APISuccess<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        match self.status {
            AppSuccess::UserLoggedIn => (StatusCode::OK, HEADER, Json(self)).into_response(),
            AppSuccess::UserCreated => (StatusCode::CREATED, HEADER, Json(self)).into_response(),
        }
    }
}
