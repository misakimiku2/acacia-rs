use crate::base::config::CONFIG;
use crate::routes::api;

pub async fn listen_api_server() {
    let addr = format!("{}:{}", CONFIG.game_server.host, 52000);
    let app = api::routes();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap_or_else(|err| {
        panic!("ApiServer failed to bind addr {}: {}", addr, err);
    });
    tracing::info!("ApiServer listening on {}", addr);
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("ApiServer server error: {}", e);
    }
}
