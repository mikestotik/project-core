use sea_orm::{Database, DatabaseConnection};

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
    Database::connect(database_url).await.expect("Failed to connect to the database")
}