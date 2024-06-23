use actix_web::web;

use crate::presentation::controllers::role_controller;
use crate::presentation::middleware::auth_middleware::AuthMiddleware;


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/roles")
            .wrap(AuthMiddleware)
            .route(web::get().to(role_controller::get_all_roles)),
    );
}
