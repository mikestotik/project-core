#[derive(Debug)]
pub enum UserRoleEnum {
    SuperAdmin,
    Admin,
    User,
}


impl UserRoleEnum {
    pub fn value(&self) -> i32 {
        match self {
            UserRoleEnum::SuperAdmin => 1,
            UserRoleEnum::Admin => 2,
            UserRoleEnum::User => 3,
        }
    }
}