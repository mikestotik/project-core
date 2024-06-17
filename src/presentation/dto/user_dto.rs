use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserDTO {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub logo: Option<String>,
    pub lang: Option<String>,
    pub role_id: Option<i32>,
}
