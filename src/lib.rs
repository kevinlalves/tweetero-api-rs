use std::net::TcpListener;
use actix_web::{App, HttpServer, dev::Server};

pub mod config;
pub mod controllers;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod utils;
pub mod schema;
pub mod models;

pub fn start_server(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().configure(routes::app))
        .listen(listener)?
        .run();

    Ok(server)
}
