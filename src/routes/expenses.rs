use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::expenses::*;

use super::AppState;

pub fn routes() -> Router<AppState> {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET])
        .allow_origin(Any);

    Router::new()
        .route("/expenses", post(create_expenses))
        .route("/expenses/category", get(get_category))
        .layer(cors)
}
