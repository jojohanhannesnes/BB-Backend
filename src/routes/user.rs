use axum::{
    http::Method,
    routing::{delete, get, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::user::*;

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    Router::new()
        .route("/api/user/:uuid", delete(delete_user))
        .route("/api/user/:uuid", put(update_user))
        .route("/api/users", get(list_user))
        .route("/api/user/dashboard", get(dashboard_user))
        .layer(cors)
}
