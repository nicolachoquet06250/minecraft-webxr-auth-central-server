use sea_orm_migration::{prelude::*, sea_orm::DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let is_mysql = manager.get_database_backend() == DatabaseBackend::MySql;

        let mut visits = Table::create();
        visits
            .table(ServerVisit::Table)
            .if_not_exists()
            .col(ColumnDef::new(ServerVisit::Id).string_len(36).not_null().primary_key())
            .col(ColumnDef::new(ServerVisit::UserId).string_len(36).not_null())
            .col(ColumnDef::new(ServerVisit::ServerId).string_len(36).not_null())
            .col(ColumnDef::new(ServerVisit::ServerUrl).string_len(255).not_null())
            .col(ColumnDef::new(ServerVisit::VisitedAt).date_time().not_null().default(Expr::current_timestamp()))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_server_visit_user")
                    .from(ServerVisit::Table, ServerVisit::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_server_visit_server")
                    .from(ServerVisit::Table, ServerVisit::ServerId)
                    .to(Server::Table, Server::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            );

        if is_mysql {
            visits.engine("InnoDB").character_set("utf8mb4").collate("utf8mb4_unicode_ci");
        }

        manager.create_table(visits.to_owned()).await?;
        manager.create_index(Index::create().name("idx_server_visit_user_at").table(ServerVisit::Table).col(ServerVisit::UserId).col(ServerVisit::VisitedAt).to_owned()).await?;
        manager.create_index(Index::create().name("idx_server_visit_user_server").table(ServerVisit::Table).col(ServerVisit::UserId).col(ServerVisit::ServerId).unique().to_owned()).await?;

        let mut favorites = Table::create();
        favorites
            .table(ServerFavorite::Table)
            .if_not_exists()
            .col(ColumnDef::new(ServerFavorite::Id).string_len(36).not_null().primary_key())
            .col(ColumnDef::new(ServerFavorite::UserId).string_len(36).not_null())
            .col(ColumnDef::new(ServerFavorite::ServerId).string_len(36).not_null())
            .col(ColumnDef::new(ServerFavorite::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_server_favorite_user")
                    .from(ServerFavorite::Table, ServerFavorite::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_server_favorite_server")
                    .from(ServerFavorite::Table, ServerFavorite::ServerId)
                    .to(Server::Table, Server::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            );

        if is_mysql {
            favorites.engine("InnoDB").character_set("utf8mb4").collate("utf8mb4_unicode_ci");
        }

        manager.create_table(favorites.to_owned()).await?;
        manager.create_index(Index::create().name("idx_server_favorite_user_at").table(ServerFavorite::Table).col(ServerFavorite::UserId).col(ServerFavorite::CreatedAt).to_owned()).await?;
        manager.create_index(Index::create().name("idx_server_favorite_user_server").table(ServerFavorite::Table).col(ServerFavorite::UserId).col(ServerFavorite::ServerId).unique().to_owned()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ServerFavorite::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ServerVisit::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum ServerVisit {
    Table,
    Id,
    UserId,
    ServerId,
    ServerUrl,
    VisitedAt,
}

#[derive(DeriveIden)]
enum ServerFavorite {
    Table,
    Id,
    UserId,
    ServerId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Server {
    Table,
    Id,
}
