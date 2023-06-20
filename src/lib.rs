use axum::routing::{get, IntoMakeService, Router};
use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;

mod users;

pub type Server = axum::Server<AddrIncoming, IntoMakeService<Router>>;

pub async fn create_server(address: SocketAddr) -> Server {
  let users_router = users::create_users_router().await;

  let app = Router::new()
    .route("/api/health", get(|| async { "OK" }))
    .nest("/api/users", users_router);

  axum::Server::bind(&address).serve(app.into_make_service())
}
