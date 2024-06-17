use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::settings::Config;


pub async fn connect_to_db(config: &Config) -> DatabaseConnection {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.postgres.user,
        config.postgres.password,
        config.postgres.host,
        config.postgres.port,
        config.postgres.database
    );

    let mut opt = ConnectOptions::new(database_url);
    opt
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .set_schema_search_path("public");

    Database::connect(opt).await.expect("Failed to connect to the database")
}