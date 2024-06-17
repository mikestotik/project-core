use actix_web::web;

use crate::presentation::controllers::auth_controller;


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(auth_controller::login))
    );
}
