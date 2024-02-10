use rocket::get;

pub const HELLO_PATH: &str = "/hello";
pub const HELLO_RESPONSE: &str = "Hello, Rocket!";
#[get("/hello")]
pub fn get_hello_endpoint() -> &'static str {
    HELLO_RESPONSE
}
