use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::user;
use crate::update_fields;


#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct CreateUserDTO {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct UpdateUserDTO {
    #[validate(email(message = "Email is not valid"))]
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub logo: Option<String>,
    pub lang: Option<String>,
    pub role_id: Option<i32>,
}

impl UpdateUserDTO {
    pub fn to_active_model(self, mut existing_user: user::ActiveModel) -> user::ActiveModel {
        update_fields!(self, existing_user, {
            username,
            email,
            password,
            logo,
            lang,
            role_id
        });
        existing_user.updated = Set(Utc::now().naive_utc());
        existing_user
    }
}
