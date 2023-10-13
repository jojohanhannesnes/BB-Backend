use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::{Method, Uri};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

pub fn init_log() {
    tracing_subscriber::fmt()
        .with_target(false)
        .json()
        .pretty()
        .init();
}
#[derive(Serialize, Deserialize)]
struct Log<T> {
    request_id: String,
    timestamp: String,
    user_id: Option<Uuid>,
    http_path: String,
    http_method: String,
    body: T,
}
//TODO learn and create macros for general log, to be called in the project
pub fn log_request<T: serde::Serialize>(
    message: impl Into<String>,
    uri: &Uri,
    method: &Method,
    user_id: Option<Uuid>,
    body: T,
) {
    let request_id = Uuid::new_v4().to_string();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    let log = Log {
        request_id,
        timestamp,
        user_id,
        http_path: uri.to_string(),
        http_method: method.to_string(),
        body,
    };
    info!("{} - {}", message.into(), json!(log));
}
