use rocket::{post, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::database::{custom_pool::CustomDbPool, thruster::insert_rocket_thruster};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RocketThrusterPostDto {
    pub name: String,
    pub manufacturer: String,
    pub min_consumption_in_liter_per_second: f64,
    pub max_consumption_in_liter_per_second: f64,
    pub fuel_type: String,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RocketThrusterGetDto {
    pub id: i64,
    pub name: String,
    pub manufacturer: String,
    pub min_consumption_in_liter_per_second: f64,
    pub max_consumption_in_liter_per_second: f64,
    pub fuel_type: String,
}


pub const THRUSTER_PATH: &str = "/thruster";

#[post("/thruster", format = "json", data = "<data>")]
pub async fn post_thruster_endpoint(
    data: Json<RocketThrusterPostDto>,
    db: &State<CustomDbPool>,
) -> Result<Json<RocketThrusterGetDto>, String> {
    match insert_rocket_thruster(db.inner(), &data).await {
        Ok(response) => Ok(Json::from(response)),
        Err(e) => Err(format!("Failed to insert thruster data: {}", e)),
    }
}
