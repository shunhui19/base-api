use salvo::prelude::*;

mod config;
mod db;
mod error;
mod hoops;

#[tokio::main]
async fn main() {
    config::init();
    let config = config::get();

    // log init.
    let _guard = config.log.guard();
    tracing::info!("log level: {}", config.log.filter_level);

    let router = Router::new().get(hello);
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn hello() -> &'static str {
    "hello, rust"
}
