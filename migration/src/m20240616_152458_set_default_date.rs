use sea_orm_migration::prelude::*;

use crate::sea_orm::Statement;


#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute(Statement::from_string(
            manager.get_database_backend(),
            "INSERT INTO roles (name) VALUES ('SUPERADMIN'), ('ADMIN'), ('USER');".to_owned(),
        )).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute(Statement::from_string(
            manager.get_database_backend(),
            "DELETE FROM roles WHERE name IN ('SUPERADMIN', 'ADMIN', 'USER');".to_owned(),
        )).await?;

        Ok(())
    }
}
