use axum::routing::{get, IntoMakeService};
use axum::Router;
use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;

pub type Server = axum::Server<AddrIncoming, IntoMakeService<Router>>;

pub fn create_server(address: SocketAddr) -> Server {
  let app = Router::new().route("/", get(|| async { "Hello, World!" }));
  axum::Server::bind(&address).serve(app.into_make_service())
}
