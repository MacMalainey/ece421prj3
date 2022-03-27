#[cfg_attr(feature = "rocket", macro_use)]
#[cfg(feature = "rocket")] extern crate rocket;
#[cfg_attr(feature = "database", macro_use)]
#[cfg(feature = "database")] extern crate diesel;
#[cfg_attr(feature = "run_migrations", macro_use)]
#[cfg(feature = "run_migrations")] extern crate diesel_migrations;

pub mod types;
#[cfg(feature = "database")]
pub mod models;
#[cfg(feature = "database")]
pub mod schema;
#[cfg(feature = "database")]
pub mod queries;