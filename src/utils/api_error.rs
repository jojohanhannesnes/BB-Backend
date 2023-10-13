use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Serialize, Deserialize};
use serde_json::json;

pub type ResultCustom<T> = core::result::Result<T, APIError>;

pub struct APIError {
    pub message: String,
    pub status_code: StatusCode,
    pub error_code: Option<i8>,
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let status_code = self.status_code;
        (status_code, 
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({"StatusCode": self.status_code.as_u16(), "ErrorCode": self.error_code, "Message": self.message}))
        ).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APISuccess<T> {
    message: String,
    data: T,
}

impl <T> APISuccess<T> {
    pub fn new(message: impl Into<String>, data: T) -> Self {
        APISuccess { message: message.into(), data }
    }
}

impl <T> IntoResponse for APISuccess<T> 
where 
T: serde::Serialize {
    fn into_response(self) -> Response {
        (StatusCode::OK,[(header::CONTENT_TYPE, "application/json")], Json(self)).into_response()
    }
}

