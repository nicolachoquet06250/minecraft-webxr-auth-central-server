use sea_orm_migration::{prelude::*, sea_orm::Database};
use std::{env, process};
use dotenvy::dotenv;

#[async_std::main]
async fn main() {
    dotenv().ok();

    if let Err(error) = run().await {
        eprintln!("{error}");
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database URL: {database_url}");
    let command = env::args().nth(1).unwrap_or_else(|| "up".to_string());
    let db = Database::connect(&database_url).await?;

    if command == "up" {
        migration::Migrator::up(&db, None).await?;
        return Ok(());
    }

    if command == "down" {
        migration::Migrator::down(&db, Some(1)).await?;
        return Ok(());
    }

    if command == "status" {
        migration::Migrator::status(&db).await?;
        return Ok(());
    }

    Err("supported migration commands: up, down, status".into())
}
