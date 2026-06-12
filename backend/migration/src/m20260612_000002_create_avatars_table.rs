use sea_orm_migration::{prelude::*, sea_orm::DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let is_mysql = manager.get_database_backend() == DatabaseBackend::MySql;

        let mut table = Table::create();
        table
            .table(Avatar::Table)
            .if_not_exists()
            .col(ColumnDef::new(Avatar::Id).string_len(36).not_null().primary_key())
            .col(ColumnDef::new(Avatar::UserId).string_len(36).not_null())
            .col(ColumnDef::new(Avatar::Name).string_len(80).not_null())
            .col(ColumnDef::new(Avatar::BaseKind).string_len(20).not_null())
            .col(ColumnDef::new(Avatar::IsActive).boolean().not_null().default(false))
            .col(ColumnDef::new(Avatar::TextureData).text().not_null())
            .col(ColumnDef::new(Avatar::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
            .col(ColumnDef::new(Avatar::UpdatedAt).date_time().not_null().default(Expr::current_timestamp()))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_avatar_user")
                    .from(Avatar::Table, Avatar::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            );

        if is_mysql {
            table
                .engine("InnoDB")
                .character_set("utf8mb4")
                .collate("utf8mb4_unicode_ci");
        }

        manager.create_table(table.to_owned()).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_avatar_user_id")
                    .table(Avatar::Table)
                    .col(Avatar::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_avatar_user_active")
                    .table(Avatar::Table)
                    .col(Avatar::UserId)
                    .col(Avatar::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_avatar_user_active")
                    .table(Avatar::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_avatar_user_id")
                    .table(Avatar::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Avatar::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Avatar {
    Table,
    Id,
    UserId,
    Name,
    BaseKind,
    IsActive,
    TextureData,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
