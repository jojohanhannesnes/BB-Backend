use axum::{Router, http::Method, routing::{delete, put,get}};
use tower_http::cors::{ CorsLayer, Any};

use crate::handlers::user::*;


pub fn routes() -> Router {
    let cors = CorsLayer::new().allow_methods([Method::GET, Method::PUT, Method::DELETE]).allow_origin(Any);

    Router::new()
    .route("/api/user/:uuid", delete(delete_user))
    .route("/api/user/:uuid", put(update_user))
    .route("/api/users", get(list_user))
    .layer(cors)
}