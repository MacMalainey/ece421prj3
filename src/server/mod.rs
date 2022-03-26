mod models;
mod routes;
mod schema;
mod requests;
mod types;
pub mod database;

pub fn get_routes() -> Vec<rocket::Route> {
    routes::get_routes()
}