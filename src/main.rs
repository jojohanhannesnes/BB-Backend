use tracing::info;

use crate::{
    routes::init_router,
    utils::{cron::init_cron, log::init_log},
};
mod handlers;
mod models;
mod routes;
mod utils;

fn initialize() {
    init_cron();
    init_log();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(init_router().await.into_make_service())
        .await?;
    Ok(())
}
