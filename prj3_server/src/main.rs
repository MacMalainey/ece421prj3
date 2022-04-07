#[macro_use] extern crate rocket;

use diesel::Connection;
use diesel::SqliteConnection;

use rocket_sync_db_pools::{diesel, database};

#[database("user_database")]
pub struct UserDbConn(SqliteConnection);

mod routes;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
        .attach(UserDbConn::fairing())
        .mount("/api/v1/", routes::get_routes());

    #[cfg(feature = "build_database")]
    {
        println!("Running Database Migration ENABLED (performing now)");
        let database_url: String = rocket.figment().extract_inner("databases.user_database.url").unwrap();
        let db_conn = SqliteConnection::establish(&database_url).expect("Unable to connect to database for initialization");
        shared_types::queries::run_migrations(&db_conn).expect("Database migration failed");
    }

    #[cfg(not(feature = "build_database"))]
    println!("Running Database Migration DISABLED (skipping)");

    rocket
}
