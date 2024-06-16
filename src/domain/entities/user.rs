use sea_orm::entity::prelude::*;
use serde::Serialize;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub logo: Option<String>,
    pub lang: Option<String>,
    pub role_id: i32,
    #[sea_orm(on_insert = "current_timestamp")]
    pub created: DateTimeWithTimeZone,
    #[sea_orm(on_update = "current_timestamp")]
    pub updated: DateTimeWithTimeZone,
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
