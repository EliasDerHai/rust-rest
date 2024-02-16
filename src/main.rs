#[macro_use]
extern crate rocket;

use rust_rest::{migrate_db_and_add_routes};

#[launch]
fn boot() -> _ {
    migrate_db_and_add_routes(rocket::build())
}