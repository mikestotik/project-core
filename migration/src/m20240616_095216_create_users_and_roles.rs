use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(User::Username).string().not_null().unique_key())
                .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                .col(ColumnDef::new(User::Password).string().not_null())
                .col(ColumnDef::new(User::Logo).string())
                .col(ColumnDef::new(User::Lang).string())
                .col(ColumnDef::new(User::Created).timestamp().not_null().default(Expr::current_timestamp()))
                .col(ColumnDef::new(User::Updated).timestamp().not_null().default(Expr::current_timestamp()))
                .to_owned(),
        )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Role::Name).string().not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        // manager.exec_stmt(
        //     Statement::from_string(
        //         DatabaseBackend::Postgres,
        //         "INSERT INTO roles (name) VALUES ('Superadmin'), ('Admin'), ('User');",
        //     )
        // ).await?;
        //
        // manager.exec_stmt(
        //     Statement::from_string(
        //         DatabaseBackend::Postgres,
        //         "INSERT INTO users (username, email, password, created, updated) VALUES ('superadmin', 'superadmin@example.com', 'hashed_password', now(), now());".to_owned(),
        //     )
        // ).await?;
        //
        // manager.exec_stmt(
        //     Statement::from_string(
        //         DatabaseBackend::Postgres,
        //         "INSERT INTO user_roles (user_id, role_id) VALUES ((SELECT id FROM users WHERE username = 'superadmin'), (SELECT id FROM roles WHERE name = 'Superadmin'));".to_owned(),
        //     )
        // ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;

        Ok(())
    }
}


#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
    Username,
    Email,
    Password,
    Logo,
    Lang,
    Created,
    Updated,
}


#[derive(DeriveIden)]
enum Role {
    #[sea_orm(iden = "roles")]
    Table,
    Id,
    Name,
}