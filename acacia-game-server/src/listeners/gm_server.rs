use std::net::SocketAddr;

use crate::base::config::CONFIG;
use crate::routes::gm;

pub async fn listen_gm_server() {
    let addr = format!("{}:{}", CONFIG.game_server.host, 1234);
    let app = gm::routes();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap_or_else(|err| {
        panic!("GMServer failed to bind addr {}: {}", addr, err);
    });
    tracing::info!("GMServer listening on {}", addr);
    if let Err(e) = axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await {
        tracing::error!("GMServer server error: {}", e);
    }
}
