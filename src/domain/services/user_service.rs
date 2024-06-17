use sea_orm::DbErr;

use crate::domain::entities::user::Model as UserModel;
use crate::enums::user_enum::UserRoleEnum;
use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::presentation::dto::user_dto::{CreateUserDTO, UpdateUserDTO};


#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}


impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }


    pub async fn find_all(&self) -> Result<Vec<UserModel>, DbErr> {
        self.repo.find_all().await
    }


    pub async fn find_one(&self, id: i32) -> Result<Option<UserModel>, DbErr> {
        self.repo.find_one(id).await
    }


    pub async fn create(&self, dto: CreateUserDTO) -> Result<UserModel, DbErr> {
        self.repo.create(dto, UserRoleEnum::USER).await
    }


    pub async fn update(&self, id: i32, dto: UpdateUserDTO) -> Result<UserModel, DbErr> {
        self.repo.update(id, dto).await
    }


    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserModel>, DbErr> {
        self.repo.find_by_username(username).await
    }


    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, DbErr> {
        self.repo.find_by_email(email).await
    }
}
