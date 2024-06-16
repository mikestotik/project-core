use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sea_orm::ActiveValue::Set;

use crate::domain::entities::user::ActiveModel as UserActiveModel;
use crate::domain::services::user_service::UserService;


#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}


pub async fn create_user(service: web::Data<UserService>, user_data: web::Json<CreateUserRequest>) -> impl Responder {
    let user = UserActiveModel {
        id: Set(1),
        username: Set(user_data.username.clone()),
        email: Set(user_data.email.clone()),
        password: Set(user_data.password.clone()),  // Hash the password before saving in a real application
        logo: Set(None),
        lang: Set(None),
        role_id: Set(3), // Default role is USER
        created: Set(Utc::now().into()),
        updated: Set(Utc::now().into()),
    };

    match service.create_user(user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
