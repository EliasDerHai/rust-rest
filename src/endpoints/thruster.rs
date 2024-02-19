use rocket::{get, post, serde::json::Json, State};
use rocket::serde::{Deserialize, Serialize};

use crate::database::{custom_pool::CustomDbPool, thruster::insert_rocket_thruster};
use crate::database::thruster::get_rocket_thrusters;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RocketThrusterEntity {
    pub id: i64,
    pub name: String,
    pub manufacturer: String,
    pub min_consumption_in_liter_per_second: f64,
    pub max_consumption_in_liter_per_second: f64,
    pub fuel_type: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RocketThruster {
    pub name: String,
    pub manufacturer: String,
    pub min_consumption_in_liter_per_second: f64,
    pub max_consumption_in_liter_per_second: f64,
    pub fuel_type: String,
}

impl PartialEq for RocketThrusterEntity {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.manufacturer == other.manufacturer &&
            (self.min_consumption_in_liter_per_second - other.min_consumption_in_liter_per_second).abs() < f64::EPSILON &&
            (self.max_consumption_in_liter_per_second - other.max_consumption_in_liter_per_second).abs() < f64::EPSILON &&
            self.fuel_type == other.fuel_type
    }
}

impl PartialEq for RocketThruster {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.manufacturer == other.manufacturer &&
            (self.min_consumption_in_liter_per_second - other.min_consumption_in_liter_per_second).abs() < f64::EPSILON &&
            (self.max_consumption_in_liter_per_second - other.max_consumption_in_liter_per_second).abs() < f64::EPSILON &&
            self.fuel_type == other.fuel_type
    }
}

pub const THRUSTER_PATH: &str = "/thruster";

#[post("/thruster", format = "json", data = "<data>")]
pub async fn post_thruster_endpoint(
    data: Json<RocketThruster>,
    db: &State<CustomDbPool>,
) -> Result<Json<RocketThrusterEntity>, String> {
    match insert_rocket_thruster(db.inner(), &data).await {
        Ok(response) => Ok(Json::from(response)),
        Err(e) => Err(format!("Failed to insert thruster data: {}", e)),
    }
}

pub const THRUSTERS_PATH: &str = "/thrusters";

#[get("/thrusters", format = "json")]
pub async fn get_thrusters_endpoint(
    db: &State<CustomDbPool>,
) -> Result<Json<Vec<RocketThrusterEntity>>, String> {
    match get_rocket_thrusters(db.inner()).await {
        Ok(response) => Ok(Json::from(response)),
        Err(e) => Err(format!("Failed to insert thruster data: {}", e)),
    }
}
