use axum::{http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::expenses::*;

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET])
        .allow_origin(Any);

    Router::new()
        .route("/api/expenses", post(create_expenses))
        // .route("/api/expenses/category", get(get_category))
        .layer(cors)
}
