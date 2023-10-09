use axum::{http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::auth::*;

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods(Method::POST)
        .allow_origin(Any);

    Router::new()
        .route("/api/login", post(login_user))
        .route("/api/register", post(create_user))
        .layer(cors)
}
