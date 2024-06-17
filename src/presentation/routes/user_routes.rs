use actix_web::web;

use crate::presentation::controllers::user_controller;


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(user_controller::get_all))
            .route("", web::post().to(user_controller::create))
            .route("/{id}", web::get().to(user_controller::get_one))
            .route("/{id}", web::put().to(user_controller::update))
            // .route("/{id}", web::delete().to(controller::delete_user))
    );
}
