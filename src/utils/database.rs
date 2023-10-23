use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

use crate::utils;

pub async fn init_database() -> DatabaseConnection {
    let connection_string = (utils::constants::DATABASE_URL).clone();
    let mut opt = ConnectOptions::new(connection_string.clone());
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true);
    match Database::connect(opt).await {
        Ok(db) => {
            info!("Connected to database");
            db
        }
        Err(err) => panic!("Cannot connect to database {err}"),
    }
}
