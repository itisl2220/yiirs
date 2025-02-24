use salvo::prelude::*;

use crate::shared::core::config;

pub mod api;
pub mod cmd;
pub mod middleware;
pub mod model;
pub mod router;
pub mod service;

pub async fn serve() {
    let addr = config::global().get_int("app.port").unwrap_or(8000);
    let acceptor = TcpListener::new(format!("0.0.0.0:{}", addr)).bind().await;
    let mut server = Server::new(acceptor);
    // Sets the MAX_CONCURRENT_STREAMS for HTTP2
    server.http2_mut().max_concurrent_streams(None);
    server.serve(router::init()).await;
}
