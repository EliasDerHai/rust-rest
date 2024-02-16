use rocket::{error, routes};
use rocket::{Rocket, Build, fairing::{self, AdHoc}};
use crate::database::custom_pool::CustomDbPool;
use rocket_db_pools::{Database, Config as DbConfig};

pub mod database;
pub mod endpoints;

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match CustomDbPool::fetch(&rocket) {
        Some(db) => match sqlx::migrate!().run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to run database migrations: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn get_server() -> rocket::Rocket<rocket::Build> {
    let routes = routes![
        endpoints::hello::get_hello_endpoint,
        endpoints::description::get_api_description_endpoint,
        endpoints::thruster::post_thruster_endpoint,
    ];

    rocket::build()
        .attach(CustomDbPool::init())
        .attach(AdHoc::try_on_ignite("Run Migrations", run_migrations))
        .mount("/", routes)
}
