use serde::{Deserialize, Serialize};
use validator::Validate;


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
