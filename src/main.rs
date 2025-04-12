mod config;
mod services;
mod routes;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use config::App_config::create_app;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = create_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3300));
    info!("Servidor corriendo en http://{}", addr);


    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
