use std::net::TcpListener;

use tweetero_api::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:3000").expect("Failed to bind port 3000");

  start_server(listener)?.await
}
