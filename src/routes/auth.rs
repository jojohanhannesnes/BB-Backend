use axum::{Router, http::Method, routing::post};
use tower_http::cors::{ CorsLayer, Any};

use crate::handlers::auth::*;


pub fn routes() -> Router {
    let cors = CorsLayer::new().allow_methods(Method::POST).allow_origin(Any);

    Router::new()
    .route("/api/login", post(login_user))
    .route("/api/user", post(create_user))
    .layer(cors)
}