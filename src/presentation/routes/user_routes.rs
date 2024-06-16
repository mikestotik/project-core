use actix_web::web;

use crate::presentation::controllers::user_controller;


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(user_controller::create_user)),
    );
}
