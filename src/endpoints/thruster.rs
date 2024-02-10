use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RocketThruster {
    name: String,
    consumption_in_liter_per_second: f32,
}

pub const THRUSTER_PATH: &str = "/thruster";
#[post("/thruster", format = "json", data = "<data>")]
pub fn post_thruster_endpoint(data: Json<RocketThruster>) -> String {
    format!(
        "Received: Thurster named '{}' with a consumption of {} liter per second on idle engine",
        data.name, data.consumption_in_liter_per_second
    )
}
