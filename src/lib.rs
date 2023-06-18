use axum::routing::{get, IntoMakeService};
use axum::Router;
use hyper::server::conn::AddrIncoming;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub type Server = axum::Server<AddrIncoming, IntoMakeService<Router>>;

pub fn create_server(port: u16) -> Server {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(ip, port);
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    axum::Server::bind(&socket).serve(app.into_make_service())
}
