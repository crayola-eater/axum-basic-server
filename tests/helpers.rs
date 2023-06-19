use rust_axum_deployment::create_server;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const SOCKET_ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);

pub async fn setup() -> SocketAddr {
  let server = create_server(SOCKET_ADDRESS).await;
  let address = server.local_addr();
  tokio::spawn(server);
  address
}
