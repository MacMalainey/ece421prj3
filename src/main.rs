#[macro_use] extern crate rocket;

use rocket::response::Redirect;
use rocket::fs::FileServer;

mod server;

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/app/")
}

#[get("/debug/database")]
async fn debug_database(conn: server::database::UserDbConn) -> String {
    conn.run(
        |c| match c.execute_batch("INSERT INTO debug_db_exists (log_time) VALUES (CURRENT_TIMESTAMP)") {
            Ok(()) => String::from("Log entry added to debug_db_exists"),
            Err(err) => format!("Error add debug log entry: {}", err),
        }
    ).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(server::database::UserDbConn::fairing())
        .mount("/data/", server::api_routes())
        .mount("/pkg/", FileServer::from(format!("{}/pkg/", env!("CARGO_MANIFEST_DIR"))))
        .mount("/app/", FileServer::from(format!("{}/app/", env!("CARGO_MANIFEST_DIR"))))
        .mount("/", routes![index, debug_database])
}