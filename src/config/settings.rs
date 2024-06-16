use config::Config as ConfigLoader;
use config::ConfigError;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
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


#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
}


#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors: CorsConfig,
    pub ssl: SslConfig,
}


#[derive(Deserialize, Debug)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}


#[derive(Deserialize, Debug)]
pub struct SslConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
}


#[derive(Deserialize, Debug)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}


#[derive(Deserialize, Debug)]
pub struct MongoConfig {
    pub url: String,
    pub database: String,
}


#[derive(Deserialize, Debug)]
pub struct LogsConfig {
    pub level: String,
    pub output: String,
}


#[derive(Deserialize, Debug)]
pub struct MailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
}


#[derive(Deserialize, Debug)]
pub struct AuthConfig {
    pub jwt_secret: String,
}


#[derive(Deserialize, Debug)]
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
