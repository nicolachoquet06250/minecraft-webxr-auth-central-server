use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RefreshToken::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RefreshToken::Id).string_len(36).not_null().primary_key())
                    .col(ColumnDef::new(RefreshToken::TokenHash).string_len(128).not_null().unique_key())
                    .col(ColumnDef::new(RefreshToken::UserId).string_len(36).not_null())
                    .col(ColumnDef::new(RefreshToken::ExpiresAt).date_time().not_null())
                    .col(ColumnDef::new(RefreshToken::RevokedAt).date_time().null())
                    .col(ColumnDef::new(RefreshToken::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(RefreshToken::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_refresh_token_user_id")
                    .table(RefreshToken::Table)
                    .col(RefreshToken::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefreshToken::Table).if_exists().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RefreshToken {
    Table,
    Id,
    TokenHash,
    UserId,
    ExpiresAt,
    RevokedAt,
    CreatedAt,
    UpdatedAt,
}
