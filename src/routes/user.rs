use axum::{
    http::Method,
    routing::{delete, get, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::user::*;

use super::AppState;

pub fn routes() -> Router<AppState> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    Router::new()
        .route("/user/:uuid", delete(delete_user))
        .route("/user/:uuid", put(update_user))
        .route("/users", get(list_user))
        .route("/user/dashboard", get(dashboard_user))
        .layer(cors)
}
