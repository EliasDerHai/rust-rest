use crate::database::{custom_pool::CustomDbPool, thruster::insert_rocket_thruster};
use rocket::{post, serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RocketThruster {
    pub name: String,
    pub manufacturer: String,
    pub min_consumption_in_liter_per_second: f32,
    pub max_consumption_in_liter_per_second: f32,
    pub fuel_type: String,
}

pub const THRUSTER_PATH: &str = "/thruster";

#[post("/thruster", format = "json", data = "<data>")]
pub async fn post_thruster_endpoint(
    data: Json<RocketThruster>,
    db: &State<CustomDbPool>,
) -> Result<String, String> {
    match insert_rocket_thruster(db.inner(), &data).await {
        Ok(_) => Ok(format!("Thruster named '{}' successfully saved", data.name)),
        Err(e) => Err(format!("Failed to insert thruster data: {}", e)),
    }
}
