use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


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
    #[sea_orm(auto_update_time)]
    pub updated: DateTime,
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
