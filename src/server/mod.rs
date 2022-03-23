mod models;
mod user;
pub mod database;

pub fn api_routes() -> Vec<rocket::Route> {
    user::user_routes()
}