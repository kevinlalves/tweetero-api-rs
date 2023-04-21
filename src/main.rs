use actix_web::{App, HttpServer};

pub mod config;
pub mod controllers;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod utils;
pub mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::app))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
