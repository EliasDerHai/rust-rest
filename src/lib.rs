use rocket::routes;

pub mod endpoints;

// run
pub fn get_server() -> rocket::Rocket<rocket::Build> {
    let routes = routes![
        endpoints::hello::get_hello_endpoint,
        endpoints::description::get_api_description_endpoint,
        endpoints::thruster::post_thruster_endpoint
    ];
    rocket::build().mount("/", routes)
}
