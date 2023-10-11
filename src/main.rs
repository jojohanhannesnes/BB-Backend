use std::net::SocketAddr;

use axum::{middleware, Extension, Router};
use sea_orm::{Database, DatabaseConnection};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::utils::cron::init_cron;
mod handlers;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {
    let connection_string = (utils::constants::DATABASE_URL).clone();
    let db: DatabaseConnection = Database::connect(connection_string).await.unwrap();
    init_cron();
    tracing_subscriber::fmt().with_target(false).init();
    let app: Router = Router::new()
        .merge(routes::user::routes())
        .merge(routes::expenses::routes())
        .route_layer(middleware::from_fn(utils::guards::guard))
        .merge(routes::auth::routes())
        .layer(Extension(db))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
