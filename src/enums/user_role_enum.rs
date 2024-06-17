#[derive(Debug)]
pub enum UserRoleEnum {
    SUPERADMIN,
    ADMIN,
    USER,
}


impl UserRoleEnum {
    pub fn value(&self) -> i32 {
        match self {
            UserRoleEnum::SUPERADMIN => 1,
            UserRoleEnum::ADMIN => 2,
            UserRoleEnum::USER => 3,
        }
    }
}