use actix_web::{HttpResponse, Responder, web};

use crate::domain::services::role_service::RoleService;


pub async fn get_all_roles(service: web::Data<RoleService>) -> impl Responder {
    match service.get_all_roles().await {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
