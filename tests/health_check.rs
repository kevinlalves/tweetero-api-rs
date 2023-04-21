use std::net::TcpListener;

use tweetero_api::start_server;

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_server();

    let client = hyper::Client::new();
    let url = address.parse().unwrap();

    let response = client
        .get(url)
        .await
        .expect("Failed to execute GET /");

    assert!(response.status().is_success());
}

fn spawn_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = start_server(listener).expect("Failed to bind address");
    let _ = actix_web::rt::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
