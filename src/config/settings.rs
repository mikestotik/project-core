use config::Config as ConfigLoader;
use config::ConfigError;
use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub server: ServerConfig,
    pub postgres: PostgresConfig,
    pub mongo: MongoConfig,
    pub logs: LogsConfig,
    pub mail: MailConfig,
    pub auth: AuthConfig,
    pub crypto: CryptoConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors: CorsConfig,
    pub ssl: SslConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SslConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MongoConfig {
    pub url: String,
    pub database: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LogsConfig {
    pub level: String,
    pub output: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_access_ttl: usize,
    pub jwt_refresh_ttl: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CryptoConfig {
    pub hash_secret: String,
    pub salt: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config_loader = ConfigLoader::builder()
            .add_source(config::File::with_name("config/config"))
            .build()?;

        config_loader.try_deserialize()
    }
}
