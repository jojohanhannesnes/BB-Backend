use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::json;
use tracing::{debug, info, info_span};
use uuid::Uuid;

pub fn init_log() {
    tracing_subscriber::fmt()
        .with_target(false)
        .json()
        .pretty()
        .init();
}
#[derive(Serialize)]
struct Log {
    request_id: String,
    timestamp: String,
    user_id: Option<String>,
    http_path: String,
    http_method: String,
}
//TODO learn and create macros for general log, to be called in the project
pub fn log_request(uri: &Uri, method: &Method) {
    let request_id = Uuid::new_v4().to_string();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    let log = Log {
        request_id,
        timestamp,
        user_id: None,
        http_path: uri.to_string(),
        http_method: method.to_string(),
    };
    info!("LOG - {}", json!(log));
}
