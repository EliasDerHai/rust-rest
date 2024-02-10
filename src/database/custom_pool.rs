use rocket_db_pools::sqlx;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("rocket_database")]
pub struct CustomDbPool(sqlx::SqlitePool);
