use actix_web::{HttpResponse, Responder, web};
use actix_web::web::{Json, Path};

use crate::domain::services::user_service::UserService;

use super::super::dto::user_dto::{CreateUserDTO, UpdateUserDTO};


pub async fn get_all(service: web::Data<UserService>) -> impl Responder {
    match service.find_all().await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


pub async fn get_one(service: web::Data<UserService>, id: Path<i32>) -> impl Responder {
    match service.find_one(id.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


pub async fn create(service: web::Data<UserService>, data: Json<CreateUserDTO>) -> impl Responder {
    match service.create(data.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().finish(),
    }
}


pub async fn update(service: web::Data<UserService>, id: Path<i32>, data: Json<UpdateUserDTO>) -> impl Responder {
    match service.update(id.into_inner(), data.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}