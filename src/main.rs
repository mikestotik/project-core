#![allow(dead_code)]
#![allow(unused_variables)]


use actix_web::{App, HttpServer};
use actix_web::web::Data;

use crate::config::database;
use crate::config::settings::Config;
use crate::infrastructure::repositories::{
    role_repository::RoleRepository,
    user_repository::UserRepository,
};


mod config;
mod domain;
mod infrastructure;
mod presentation;
mod application;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env().expect("Failed to load configuration");
    let db = database::connect_to_db(&config).await;

    let role_repo = RoleRepository::new(db.clone());
    let user_repo = UserRepository::new(db.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(role_repo.clone()))
            .app_data(Data::new(user_repo.clone()))
            .configure(presentation::routes::init)
    })
        .bind((config.server.host.as_str(), config.server.port))?
        .run()
        .await
}