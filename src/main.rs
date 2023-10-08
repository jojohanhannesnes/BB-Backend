use axum::{middleware, Extension, Router};
use sea_orm::{Database, DatabaseConnection};
mod handlers;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    server().await;
}

async fn server() {
    let connection_string = (utils::constants::DATABASE_URL).clone();
    let db: DatabaseConnection = Database::connect(connection_string).await.unwrap();

    let app: Router = Router::new()
        .merge(routes::user::routes())
        .route_layer(middleware::from_fn(utils::guards::guard))
        .merge(routes::auth::routes())
        .layer(Extension(db));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
