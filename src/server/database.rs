use rocket_sync_db_pools::{diesel, database};

#[database("user_database")]
pub struct UserDbConn(diesel::SqliteConnection);