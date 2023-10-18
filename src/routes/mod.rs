use std::any::Any;

use axum::{
    body::Body,
    http::{header, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    Extension, Router,
};

use crate::utils::{database::init_database, guards};

pub mod auth;
pub mod expenses;
pub mod user;

pub async fn init_router() -> Router {
    Router::new()
        .merge(user::routes())
        .merge(expenses::routes())
        .route_layer(middleware::from_fn(guards::guard))
        .merge(auth::routes())
        .layer(Extension(init_database().await))
        .layer(tower_http::catch_panic::CatchPanicLayer::custom(
            handle_panic,
        ))
        .fallback(fallback)
}

async fn fallback() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };

    let body = serde_json::json!({
        "error": {
            "kind": "panic",
            "details": details,
        }
    });
    let body = serde_json::to_string(&body).unwrap();

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .unwrap()
}
