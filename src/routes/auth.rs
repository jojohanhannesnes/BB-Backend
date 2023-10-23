use axum::{http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::auth::*;

use super::AppState;

pub fn routes() -> Router<AppState> {
    let cors = CorsLayer::new()
        .allow_methods(Method::POST)
        .allow_origin(Any);

    Router::new()
        .route("/login", post(login_user))
        .route("/register", post(create_user))
        .layer(cors)
}
