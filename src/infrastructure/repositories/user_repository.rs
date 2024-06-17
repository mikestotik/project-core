use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;

use crate::domain::entities::user;
use crate::enums::user_enum::UserRoleEnum;
use crate::presentation::dto::user_dto::{CreateUserDTO, UpdateUserDTO};


#[derive(Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}


impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }


    pub async fn create(&self, user: CreateUserDTO, role: UserRoleEnum) -> Result<user::Model, DbErr> {
        let user = user::ActiveModel {
            username: Set(user.username),
            email: Set(user.email),
            password: Set(user.password),
            role_id: Set(role.value()),
            ..Default::default()
        };
        user.insert(&self.db).await
    }

    pub async fn update(&self, id: i32, dto: UpdateUserDTO) -> Result<user::Model, DbErr> {
        user::Model::update_entity(&self.db, id, dto).await
    }


    pub async fn find_all(&self) -> Result<Vec<user::Model>, DbErr> {
        user::Entity::find().all(&self.db).await
    }


    pub async fn find_one(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(id).one(&self.db).await
    }


    pub async fn find_by_username(&self, username: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find().filter(user::Column::Username.eq(username)).one(&self.db).await
    }


    pub async fn find_by_email(&self, email: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find().filter(user::Column::Email.eq(email)).one(&self.db).await
    }


    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        user::Model::delete_entity(&self.db, id).await
    }
}
