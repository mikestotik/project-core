use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
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
        let mut user: user::Model = user::Entity::find_by_id(id).one(&self.db).await?
            .ok_or(DbErr::RecordNotFound("User not found".to_owned()))?;

        if let Some(username) = dto.username {
            user.username = username;
        }
        if let Some(email) = dto.email {
            user.email = email;
        }
        if let Some(password) = dto.password {
            user.password = password;
        }
        if let Some(logo) = dto.logo {
            user.logo = Option::from(logo);
        }
        if let Some(lang) = dto.lang {
            user.lang = Option::from(lang);
        }
        if let Some(role_id) = dto.role_id {
            user.role_id = role_id;
        }
        user.updated = Utc::now().naive_utc();

        let user = user::ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            email: Set(user.email),
            password: Set(user.password),
            logo: Set(user.logo),
            lang: Set(user.lang),
            role_id: Set(user.role_id),
            updated: Set(user.updated),
            ..Default::default()
        };
        user.update(&self.db).await
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
}
