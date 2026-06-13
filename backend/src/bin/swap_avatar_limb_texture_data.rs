use dotenvy::dotenv;
use sea_orm::{ConnectionTrait, Database, Statement};
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let apply = env::args().any(|arg| arg == "--apply");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await?;
    let backend = db.get_database_backend();

    let rows = db
        .query_all(Statement::from_string(backend, "SELECT id, texture_data FROM avatar".to_string()))
        .await?;

    let mut fixed = 0usize;
    let mut skipped = 0usize;
    let mut failed = 0usize;

    for row in rows {
        let id: String = row.try_get("", "id")?;
        let texture_data: String = row.try_get("", "texture_data")?;
        let Ok(mut json) = serde_json::from_str::<Value>(&texture_data) else {
            failed += 1;
            eprintln!("[ERROR] invalid JSON for avatar {}", id);
            continue;
        };

        if !swap_limbs(&mut json) {
            skipped += 1;
            println!("[SKIP] avatar {} has incomplete texture_data", id);
            continue;
        }

        let next_texture_data = serde_json::to_string(&json)?;
        if next_texture_data == texture_data {
            skipped += 1;
            println!("[SKIP] avatar {} unchanged", id);
            continue;
        }

        fixed += 1;
        if apply {
            db.execute(Statement::from_sql_and_values(
                backend,
                "UPDATE avatar SET texture_data = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
                [next_texture_data.into(), id.clone().into()],
            ))
            .await?;
            println!("[APPLIED] avatar {}", id);
        } else {
            println!("[DRY-RUN] avatar {} would be updated", id);
        }
    }

    println!("Done: fixed={}, skipped={}, failed={}, apply={}", fixed, skipped, failed, apply);
    Ok(())
}

fn swap_limbs(value: &mut Value) -> bool {
    let Some(parts) = value.get_mut("parts").and_then(Value::as_object_mut) else {
        return false;
    };

    if !(parts.contains_key("leftArm") && parts.contains_key("rightArm") && parts.contains_key("leftLeg") && parts.contains_key("rightLeg")) {
        return false;
    }

    let left_arm = parts.remove("leftArm").unwrap();
    let right_arm = parts.remove("rightArm").unwrap();
    let left_leg = parts.remove("leftLeg").unwrap();
    let right_leg = parts.remove("rightLeg").unwrap();

    parts.insert("leftArm".to_string(), right_arm);
    parts.insert("rightArm".to_string(), left_arm);
    parts.insert("leftLeg".to_string(), right_leg);
    parts.insert("rightLeg".to_string(), left_leg);

    true
}
