use actix_web::web;

use crate::presentation::controllers::user_controller;


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(user_controller::get_all_users))
            .route("", web::post().to(user_controller::create_user))
            // .route("/{id}", web::get().to(controller::get_user))
            // .route("/{id}", web::put().to(controller::update_user))
            // .route("/{id}", web::delete().to(controller::delete_user))
    );
}
