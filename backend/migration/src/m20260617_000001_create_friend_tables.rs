use sea_orm_migration::{prelude::*, sea_orm::DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let is_mysql = manager.get_database_backend() == DatabaseBackend::MySql;

        let mut friend_requests = Table::create();
        friend_requests
            .table(FriendRequest::Table)
            .if_not_exists()
            .col(ColumnDef::new(FriendRequest::Id).string_len(36).not_null().primary_key())
            .col(ColumnDef::new(FriendRequest::RequesterUserId).string_len(36).not_null())
            .col(ColumnDef::new(FriendRequest::ReceiverUserId).string_len(36).not_null())
            .col(ColumnDef::new(FriendRequest::Status).string_len(16).not_null())
            .col(ColumnDef::new(FriendRequest::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
            .col(ColumnDef::new(FriendRequest::UpdatedAt).date_time().not_null().default(Expr::current_timestamp()))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_friend_request_requester_user")
                    .from(FriendRequest::Table, FriendRequest::RequesterUserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_friend_request_receiver_user")
                    .from(FriendRequest::Table, FriendRequest::ReceiverUserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            );

        if is_mysql {
            friend_requests.engine("InnoDB").character_set("utf8mb4").collate("utf8mb4_unicode_ci");
        }

        manager.create_table(friend_requests.to_owned()).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_friend_request_receiver_status")
                    .table(FriendRequest::Table)
                    .col(FriendRequest::ReceiverUserId)
                    .col(FriendRequest::Status)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_friend_request_requester_status")
                    .table(FriendRequest::Table)
                    .col(FriendRequest::RequesterUserId)
                    .col(FriendRequest::Status)
                    .to_owned(),
            )
            .await?;

        let mut friendships = Table::create();
        friendships
            .table(Friendship::Table)
            .if_not_exists()
            .col(ColumnDef::new(Friendship::Id).string_len(36).not_null().primary_key())
            .col(ColumnDef::new(Friendship::UserAId).string_len(36).not_null())
            .col(ColumnDef::new(Friendship::UserBId).string_len(36).not_null())
            .col(ColumnDef::new(Friendship::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_friendship_user_a")
                    .from(Friendship::Table, Friendship::UserAId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_friendship_user_b")
                    .from(Friendship::Table, Friendship::UserBId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            );

        if is_mysql {
            friendships.engine("InnoDB").character_set("utf8mb4").collate("utf8mb4_unicode_ci");
        }

        manager.create_table(friendships.to_owned()).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_friendship_user_pair")
                    .table(Friendship::Table)
                    .col(Friendship::UserAId)
                    .col(Friendship::UserBId)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_friendship_user_b")
                    .table(Friendship::Table)
                    .col(Friendship::UserBId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Friendship::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(FriendRequest::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum FriendRequest {
    Table,
    Id,
    RequesterUserId,
    ReceiverUserId,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Friendship {
    Table,
    Id,
    UserAId,
    UserBId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
