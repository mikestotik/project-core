use sea_orm::DbErr;

use crate::domain::entities::user::{ActiveModel as UserActiveModel, Model as UserModel};
use crate::infrastructure::repositories::user_repository::UserRepository;


#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}


impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, user: UserActiveModel) -> Result<UserModel, DbErr> {
        self.repo.create(user).await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserModel>, DbErr> {
        self.repo.find_by_username(username).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, DbErr> {
        self.repo.find_by_email(email).await
    }
}
