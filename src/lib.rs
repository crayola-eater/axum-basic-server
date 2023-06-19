use axum::http::StatusCode;
use axum::routing::{get, IntoMakeService, Router};
use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;

mod users;

pub type Server = axum::Server<AddrIncoming, IntoMakeService<Router>>;

pub async fn create_server(address: SocketAddr) -> Server {
  let app = Router::new().route("/api/health", get(|| async { StatusCode::OK }));

  axum::Server::bind(&address).serve(app.into_make_service())
}
