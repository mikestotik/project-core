use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
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
        ).await?;

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
                .col(ColumnDef::new(User::RoleId).integer().not_null())
                .col(ColumnDef::new(User::Created).timestamp().not_null().default(Expr::current_timestamp()))
                .col(ColumnDef::new(User::Updated).timestamp().not_null().default(Expr::current_timestamp()))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_user_role")
                        .from(User::Table, User::RoleId)
                        .to(Role::Table, Role::Id),
                )
                .to_owned(),
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Role::Table).to_owned()).await?;

        Ok(())
    }
}


#[derive(DeriveIden)]
enum Role {
    #[sea_orm(iden = "roles")]
    Table,
    Id,
    Name,
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
    RoleId,
    Created,
    Updated,
}
