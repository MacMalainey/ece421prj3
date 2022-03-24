#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use rocket::response::Redirect;
use rocket::fs::FileServer;

mod server;

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/app/")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(server::database::UserDbConn::fairing())
        .mount("/data/", server::get_routes())
        .mount("/pkg/", FileServer::from(format!("{}/pkg/", env!("CARGO_MANIFEST_DIR"))))
        .mount("/app/", FileServer::from(format!("{}/app/", env!("CARGO_MANIFEST_DIR"))))
        .mount("/", routes![index])
}