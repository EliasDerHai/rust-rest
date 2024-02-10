use crate::endpoints::thruster::RocketThruster;
use rocket::serde::json::Json;

use super::custom_pool::CustomDbPool;

pub async fn insert_rocket_thruster(
    pool: &CustomDbPool,
    thruster: &Json<RocketThruster>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO rocket_thruster (name, consumption_in_liter_per_second) VALUES (?, ?)",
        thruster.name,
        thruster.consumption_in_liter_per_second
    )
    .execute(&**pool)
    .await?;

    Ok(())
}
