use rocket::get;

pub const API_DESCRIPTION_PATH: &str = "/description";
pub const API_DESCRIPTION_RESPONSE: &str = "This REST api allows building rockets with rocket!";
#[get("/description")]
pub fn get_api_description_endpoint() -> &'static str {
    API_DESCRIPTION_RESPONSE
}
