use rocket::serde::json::Json;

use crate::endpoints::thruster::RocketThruster;

use super::custom_pool::CustomDbPool;

// pub async fn insert_rocket_thruster(
//     pool: &CustomDbPool,
//     thruster: &Json<RocketThruster>,
// ) -> Result<(), sqlx::Error> {
//     println!("REACHED!!! 1");
//     sqlx::query!(
//         "INSERT INTO rocket_thruster (name, manufacturer, min_consumption_in_liter_per_second, max_consumption_in_liter_per_second, fuel_type) VALUES (?, ?, ?, ?, ?)",
//         thruster.name,
//         thruster.manufacturer,
//         thruster.min_consumption_in_liter_per_second,
//         thruster.max_consumption_in_liter_per_second,
//         thruster.fuel_type
//     )
//     .execute(&**pool)
//     .await?;
//     println!("REACHED!!! 2");
//
//     Ok(())
// }


pub async fn insert_rocket_thruster(pool: &CustomDbPool, thruster: &Json<RocketThruster>) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO rocket_thruster (name, manufacturer, min_consumption_in_liter_per_second, max_consumption_in_liter_per_second, fuel_type) VALUES (?, ?, ?, ?, ?)",
        // thruster.name,
        // thruster.manufacturer,
        // thruster.min_consumption_in_liter_per_second,
        // thruster.max_consumption_in_liter_per_second,
        // thruster.fuel_type
    )
        .bind(&thruster.name)
        .bind(&thruster.manufacturer)
        .bind(&thruster.min_consumption_in_liter_per_second)
        .bind(&thruster.max_consumption_in_liter_per_second)
        .bind(&thruster.fuel_type)
        .execute(&**pool)
        .await;


    match result {
        Ok(_) => {
            println!("Wrote {} to DATABASE_URL: {}", thruster.name, std::env::var("DATABASE_URL").unwrap_or_default());
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to insert thruster: {:?}", e);
            Err(e)
        }
    }
}
