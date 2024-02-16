#[macro_use]
extern crate rocket;

use rust_rest::get_server;

#[launch]
fn boot() -> _ {
    get_server()
}