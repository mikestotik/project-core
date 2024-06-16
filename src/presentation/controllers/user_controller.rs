use actix_web::{HttpResponse, Responder, web};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::domain::entities::user::ActiveModel as UserActiveModel;
use crate::domain::services::user_service::UserService;
use crate::enums::user_enum::UserRoleEnum;


#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}


pub async fn get_all_users(service: web::Data<UserService>, user_data: web::Json<CreateUserRequest>) -> impl Responder {
    match service.find_all().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


pub async fn create_user(service: web::Data<UserService>, user_data: web::Json<CreateUserRequest>) -> impl Responder {
    let default_role = UserRoleEnum::USER;
    let user = UserActiveModel {
        username: Set(user_data.username.clone()),
        email: Set(user_data.email.clone()),
        password: Set(user_data.password.clone()),  // Hash the password before saving in a real application
        role_id: Set(default_role.value()),
        ..Default::default()
    };

    match service.create_user(user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
