use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::presentation::dto::user_dto::UpdateUserDTO;


#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub logo: Option<String>,
    pub lang: Option<String>,
    pub role_id: i32,
    pub created: DateTime,
    pub updated: DateTime,
}


impl Model {
    pub async fn update_user(db: &DatabaseConnection, id: i32, dto: UpdateUserDTO) -> Result<Model, DbErr> {
        let user: ActiveModel = Self::find_user_by_id(db, id).await?.into();

        let mut updated_user = user;

        dto.username.map(|username| updated_user.username = Set(username));
        dto.email.map(|email| updated_user.email = Set(email));
        dto.password.map(|password| updated_user.password = Set(password));
        dto.logo.map(|logo| updated_user.logo = Set(Some(logo)));
        dto.lang.map(|lang| updated_user.lang = Set(Some(lang)));
        dto.role_id.map(|role_id| updated_user.role_id = Set(role_id));

        updated_user.updated = Set(Utc::now().naive_utc());

        updated_user.update(db).await
    }

    async fn find_user_by_id(db: &DatabaseConnection, id: i32) -> Result<Model, DbErr> {
        Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_owned()))
    }
}


#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Role,
}


impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Role => Entity::belongs_to(super::role::Entity)
                .from(Column::RoleId)
                .to(super::role::Column::Id)
                .into(),
        }
    }
}


impl ActiveModelBehavior for ActiveModel {}
