pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_users_table;
mod m20240101_000002_create_servers_table;
mod m20260612_000001_remove_relay_domain_from_server;
mod m20260612_000002_create_avatars_table;
mod m20260614_000001_create_server_history_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_users_table::Migration),
            Box::new(m20240101_000002_create_servers_table::Migration),
            Box::new(m20260612_000001_remove_relay_domain_from_server::Migration),
            Box::new(m20260612_000002_create_avatars_table::Migration),
            Box::new(m20260614_000001_create_server_history_tables::Migration),
        ]
    }
}