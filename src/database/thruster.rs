use crate::endpoints::thruster::RocketThruster;
use rocket::serde::json::Json;

use super::custom_pool::CustomDbPool;

pub async fn insert_rocket_thruster(
    pool: &CustomDbPool,
    thruster: &Json<RocketThruster>,
) -> Result<(), sqlx::Error> {
    // sqlx::query!(
    //     "INSERT INTO rocket_thruster (name, manufacturer, min_consumption_in_liter_per_second, max_consumption_in_liter_per_second, fuel_type) VALUES (?, ?, ?, ?, ?)",
    //     thruster.name,
    //     thruster.manufacturer,
    //     thruster.min_consumption_in_liter_per_second,
    //     thruster.max_consumption_in_liter_per_second,
    //     thruster.fuel_type
    // )
    // .execute(&**pool)
    // .await?;

    Ok(())
}
