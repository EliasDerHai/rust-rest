#[macro_use]
extern crate rocket;

use rust_rest::{rocket};

#[launch]
fn boot() -> _ {
    rocket()
}