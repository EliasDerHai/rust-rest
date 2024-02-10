use rust_rest::get_server;

#[macro_use]
extern crate rocket;

#[launch]
fn boot() -> _ {
    get_server()
}
