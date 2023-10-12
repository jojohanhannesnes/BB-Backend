use axum::{http::StatusCode, middleware, response::IntoResponse, Extension, Router};

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
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
