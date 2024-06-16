use sea_orm::{DatabaseConnection, EntityTrait};

use crate::domain::entities::role;


#[derive(Clone)]
pub struct RoleRepository {
    db: DatabaseConnection,
}


impl RoleRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<role::Model>, sea_orm::DbErr> {
        role::Entity::find().all(&self.db).await
    }
}
