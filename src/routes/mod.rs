use std::any::Any;

use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    http::{header, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    BoxError, Router,
};
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tower::{
    timeout::{self, TimeoutLayer},
    ServiceBuilder,
};
use tower_http::catch_panic::CatchPanicLayer;

use crate::utils::{
    database::init_database,
    guards,
    mapper::api_error::{APIError, AppError},
};

pub mod auth;
pub mod expenses;
pub mod user;
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn init_router() -> Router {
    let app_state = AppState {
        db: init_database().await,
    };

    let routes = Router::new()
        .merge(user::routes())
        .merge(expenses::routes())
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            guards::guard,
        ))
        .merge(auth::routes());

    let services = ServiceBuilder::new()
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(HandleErrorLayer::new(handle_timeout_error))
        .layer(TimeoutLayer::new(Duration::from_secs(2)));

    Router::new()
        .nest("/api", routes)
        .with_state(app_state)
        .layer(services)
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

async fn handle_timeout_error(err: BoxError) -> APIError {
    if err.is::<timeout::error::Elapsed>() {
        APIError::new(AppError::RequestTimeout, err.to_string())
    } else {
        APIError::new(AppError::InternalError, err.to_string())
    }
}
