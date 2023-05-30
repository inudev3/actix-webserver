pub mod schema;

pub mod db_connection;
pub mod models;
pub mod handlers;
pub mod errors;
pub mod utils;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate dotenv;