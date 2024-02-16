#[macro_use]
extern crate rocket;

use rust_rest::{CustomizedRocket};

#[launch]
fn boot() -> _ {
    rocket::build().take_off()
}