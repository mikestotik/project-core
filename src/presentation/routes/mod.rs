use actix_web::web;
use actix_web::web::ServiceConfig;


pub mod user_routes;
pub mod role_routes;


pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v1.0")
            .configure(user_routes::init)
            .configure(role_routes::init),
    );
}