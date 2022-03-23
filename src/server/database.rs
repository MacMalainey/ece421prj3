use rocket_sync_db_pools::{rusqlite, database};

#[database("user_database")]
pub struct UserDbConn(rusqlite::Connection);