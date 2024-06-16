pub use sea_orm_migration::prelude::*;


mod m20240616_095216_create_users_and_roles;
mod m20240616_152458_set_default_date;


pub struct Migrator;


#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240616_095216_create_users_and_roles::Migration),
            Box::new(m20240616_152458_set_default_date::Migration),
        ]
    }
}
