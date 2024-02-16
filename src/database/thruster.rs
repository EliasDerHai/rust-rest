use rocket::serde::json::Json;

use crate::endpoints::thruster::{RocketThruster, RocketThrusterEntity};

use super::custom_pool::CustomDbPool;

pub async fn insert_rocket_thruster(
    pool: &CustomDbPool,
    thruster: &Json<RocketThruster>,
) -> Result<RocketThrusterEntity, sqlx::Error> {
    let mut transaction = pool.begin().await?;

    sqlx::query!(
        "INSERT INTO rocket_thruster (name, manufacturer, min_consumption_in_liter_per_second, max_consumption_in_liter_per_second, fuel_type) VALUES (?, ?, ?, ?, ?)",
        thruster.name,
        thruster.manufacturer,
        thruster.min_consumption_in_liter_per_second,
        thruster.max_consumption_in_liter_per_second,
        thruster.fuel_type
    )
        .execute(&mut *transaction)
        .await?;

    let inserted_thruster = sqlx::query_as!(
        RocketThrusterEntity,
        "SELECT id, name, manufacturer, min_consumption_in_liter_per_second, max_consumption_in_liter_per_second, fuel_type FROM rocket_thruster WHERE id = last_insert_rowid()"
    )
        .fetch_one(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(inserted_thruster)
}

pub async fn get_rocket_thrusters(
    pool: &CustomDbPool
) -> Result<Vec<RocketThrusterEntity>, sqlx::Error> {
    sqlx::query_as!(
        RocketThrusterEntity,
        "SELECT * FROM rocket_thruster")
        .fetch_all(&**pool)
        .await
}