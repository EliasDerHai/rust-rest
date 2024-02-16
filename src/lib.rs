use rocket::{error, routes};
use rocket::{Build, fairing::{self, AdHoc}, Rocket};
use rocket_db_pools::Database;

use crate::database::custom_pool::CustomDbPool;

pub mod database;
pub mod endpoints;

pub trait CustomizedRocket {
    fn take_off(self) -> Rocket<Build>;
}


pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    // Assuming `CustomDbPool` is compatible with `fetch` to get the pool.
    if let Some(db) = CustomDbPool::fetch(&rocket) {
        if let Err(e) = sqlx::migrate!().run(&**db).await {
            error!("Failed to run database migrations: {}", e);
            return Err(rocket);
        }
    }

    Ok(rocket)
}

impl CustomizedRocket for Rocket<Build> {
    fn take_off(self) -> Rocket<Build> {
        self
            .attach(CustomDbPool::init())
            .attach(AdHoc::try_on_ignite("Run Migrations", run_migrations))
            .mount("/", routes![
            endpoints::hello::get_hello_endpoint,
            endpoints::description::get_api_description_endpoint,
            endpoints::thruster::post_thruster_endpoint,
        ])
    }
}