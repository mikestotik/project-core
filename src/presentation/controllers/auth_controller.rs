use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::config::auth::create_jwt;
use crate::config::settings::Config;
use crate::domain::services::user_service::UserService;


#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn login(
    service: web::Data<UserService>,
    auth_data: web::Json<LoginRequest>,
    config: web::Data<Config>,
) -> impl Responder {
    match service.find_by_email(&auth_data.email).await {
        Ok(Some(user)) => {
            if user.password == auth_data.password {
                let access_token = create_jwt(
                    &user.id.to_string(),
                    &user.role_id.to_string(),
                    &config.auth.jwt_secret,
                    &config.auth.jwt_access_ttl,
                );
                let refresh_token = create_jwt(
                    &user.id.to_string(),
                    &user.role_id.to_string(),
                    &config.auth.jwt_secret,
                    &config.auth.jwt_refresh_ttl,
                );
                HttpResponse::Ok().json(LoginResponse {
                    access_token, 
                    refresh_token
                })
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
        _ => HttpResponse::Unauthorized().finish(),
    }
}
