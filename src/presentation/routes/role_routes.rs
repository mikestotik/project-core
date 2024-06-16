use actix_web::web;

use crate::presentation::controllers::role_controller;


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/roles")
            .route(web::get().to(role_controller::get_all_roles)),
    );
}
