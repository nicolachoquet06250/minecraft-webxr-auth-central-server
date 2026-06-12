use sea_orm_migration::{prelude::*, sea_orm::DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let is_mysql = manager.get_database_backend() == DatabaseBackend::MySql;

        let mut updated_at = ColumnDef::new(Server::UpdatedAt);
        updated_at.date_time().not_null().default(Expr::current_timestamp());

        if is_mysql {
            updated_at.extra("ON UPDATE CURRENT_TIMESTAMP");
        }

        let mut table = Table::create();
        table
            .table(Server::Table)
            .if_not_exists()
            .col(ColumnDef::new(Server::Id).char_len(36).not_null().primary_key())
            .col(ColumnDef::new(Server::OwnerId).char_len(36).not_null())
            .col(ColumnDef::new(Server::Name).string_len(255).not_null())
            .col(ColumnDef::new(Server::GameDomain).string_len(255).not_null().unique_key())
            .col(ColumnDef::new(Server::Description).text())
            .col(ColumnDef::new(Server::IsActive).boolean().not_null().default(true))
            .col(ColumnDef::new(Server::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
            .col(&mut updated_at)
            .foreign_key(
                ForeignKey::create()
                    .name("fk_servers_owner_id")
                    .from(Server::Table, Server::OwnerId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            );

        if is_mysql {
            table
                .col(ColumnDef::new(Server::RelayDomain).string_len(255).not_null().unique_key())
                .engine("InnoDB")
                .character_set("utf8mb4")
                .collate("utf8mb4_unicode_ci");
        }

        manager.create_table(table.to_owned()).await?;

        manager.create_index(Index::create().name("idx_owner_id").table(Server::Table).col(Server::OwnerId).to_owned()).await?;
        manager.create_index(Index::create().name("idx_game_domain").table(Server::Table).col(Server::GameDomain).to_owned()).await?;

        if is_mysql {
            manager.create_index(Index::create().name("idx_relay_domain").table(Server::Table).col(Server::RelayDomain).to_owned()).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Server::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Server {
    Table,
    Id,
    OwnerId,
    Name,
    RelayDomain,
    GameDomain,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
