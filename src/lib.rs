use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

// endpoints
pub const HELLO_PATH: &str = "/hello";
pub const HELLO_RESPONSE: &str = "Hello, Rocket!";
#[get("/hello")]
fn get_hello_endpoint() -> &'static str {
    HELLO_RESPONSE
}

pub const API_DESCRIPTION_PATH: &str = "/description";
pub const API_DESCRIPTION_RESPONSE: &str = "This REST api allows building rockets with rocket!";
#[get("/description")]
fn get_api_description_endpoint() -> &'static str {
    API_DESCRIPTION_RESPONSE
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct RocketThruster {
    name: String,
    consumption_in_liter_per_second: f32,
}

pub const THRUSTER_PATH: &str = "/thruster";
#[post("/thruster", format = "json", data = "<data>")]
fn post_thruster_endpoint(data: Json<RocketThruster>) -> String {
    format!(
        "Received: Thurster named '{}' with a consumption of {} liter per second on idle engine",
        data.name, data.consumption_in_liter_per_second
    )
}

// run
pub fn get_server() -> rocket::Rocket<rocket::Build> {
    let routes = routes![
        get_hello_endpoint,
        get_api_description_endpoint,
        post_thruster_endpoint
    ];
    rocket::build().mount("/", routes)
}
