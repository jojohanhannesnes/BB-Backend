use std::error::Error;

use migration::{Migrator, MigratorTrait};
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
            run_migration(&db).await.unwrap();
            info!("Connected to database {:?} :jo", connection_string);
            db
        }
        Err(err) => panic!("Cannot connect to database {err}"),
    }
}

async fn run_migration(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    Migrator::up(db, None).await?;
    Ok(())
}
