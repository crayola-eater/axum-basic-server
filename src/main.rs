use rust_axum_deployment::create_server;
use std::env;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .expect("PORT environment variable should be set.")
        .parse::<u16>()
        .expect("PORT should valid u16");

    println!("Attempting to create and bind server to port {port}");

    let server = create_server(port);

    println!("Server created, about to await");

    server.await.expect("Server should run");
}
