use rocket::routes;
use rocket_db_pools::Database;

pub mod database;
pub mod endpoints;

// run
pub fn get_server() -> rocket::Rocket<rocket::Build> {
    let routes = routes![
        endpoints::hello::get_hello_endpoint,
        endpoints::description::get_api_description_endpoint,
        endpoints::thruster::post_thruster_endpoint
    ];

    rocket::build()
        .attach(database::custom_pool::CustomDbPool::init())
        .mount("/", routes)
}
