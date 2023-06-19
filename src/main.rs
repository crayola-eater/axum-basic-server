use rust_axum_deployment::create_server;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const DEFAULT_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_PORT: u16 = 3_000;

fn create_socket_address_from_env() -> SocketAddr {
  let ip = env::var("HOST_IP").map_or(DEFAULT_ADDR, |raw| {
    raw.parse::<IpAddr>().expect("HOST_IP should be valid")
  });

  let port = env::var("PORT").map_or(DEFAULT_PORT, |raw| {
    raw.parse::<u16>().expect("PORT should be valid")
  });

  SocketAddr::new(ip, port)
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
  let socket = create_socket_address_from_env();
  println!("Attempting to create and bind server to socket {socket}");

  let server = create_server(socket).await;

  println!("Server bound to {}", server.local_addr());

  server.await
}
