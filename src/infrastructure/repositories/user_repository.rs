use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;

use crate::domain::entities::user;


#[derive(Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}


impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }


    pub async fn create(&self, user: user::ActiveModel) -> Result<user::Model, sea_orm::DbErr> {
        let result = user::Entity::insert(user.clone()).exec(&self.db).await?;
        // user::Entity::find_by_id(result.last_insert_id).one(&self.db).await?;
        Ok(user::Model {
            id: result.last_insert_id,
            username: user.username.unwrap(),
            email: user.email.unwrap(),
            password: user.password.unwrap(),
            role_id: user.role_id.unwrap(),
            // created: user.created.unwrap(),
            // updated: user.updated.unwrap(),
            ..Default::default()
        })
    }


    pub async fn find_all(&self) -> Result<Vec<user::Model>, DbErr> {
        user::Entity::find().all(&self.db).await
    }


    pub async fn find_by_id(&self, id: &i32) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find_by_id(*id).one(&self.db).await
    }


    pub async fn find_by_username(&self, username: &str) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find().filter(user::Column::Username.eq(username)).one(&self.db).await
    }


    pub async fn find_by_email(&self, email: &str) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find().filter(user::Column::Email.eq(email)).one(&self.db).await
    }
}
