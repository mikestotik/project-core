use sea_orm::DeleteResult;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
    pub async fn update_entity(
        db: &DatabaseConnection,
        id: i32,
        dto: UpdateUserDTO,
    ) -> Result<Model, DbErr> {
        if let Err(e) = dto.validate() {
            return Err(DbErr::Custom(e.to_string()));
        }

        let user: ActiveModel = Self::find_entity_by_id(db, id).await?.into();
        let updated_user = dto.to_active_model(user);

        updated_user.update(db).await
    }

    pub async fn delete_entity(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        let user: ActiveModel = Self::find_entity_by_id(db, id).await?.into();
        user.delete(db).await
    }

    async fn find_entity_by_id(db: &DatabaseConnection, id: i32) -> Result<Model, DbErr> {
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
