use std::net::SocketAddr;
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use crate::api::routes_init;
pub use self::error::Result;

mod error;
mod api;

#[tokio::main]
async fn main()-> Result<()> {

    tracing_subscriber::fmt()
        .without_time() // for early local development
        .with_target(false)
        .init();

    let router_all = Router::new().nest("/api", routes_init());

    // region:    ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    let listener = TcpListener::bind(addr).await.unwrap();
    info!(" {:<12} - {addr}\n", "LISTENING");
    axum::serve(listener, router_all).await.unwrap();
    // endregion: ---Start Server

    Ok(())
}
