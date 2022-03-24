mod models;
mod routes;
mod schema;
mod forms;
pub mod database;

pub fn get_routes() -> Vec<rocket::Route> {
    routes::get_routes()
}