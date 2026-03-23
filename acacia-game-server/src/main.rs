mod base {
    pub mod config;
    pub mod conn;
    pub mod dispatcher;
    pub mod packet;
}
mod handler {
    pub mod game_server;
}
mod func {
    mod none;
    mod req_login;
    mod req_reconnect;
    mod req_callgs;
    mod ntf_logout;
    mod ntf_setattr;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .compact()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&base::config::CONFIG.log)))
        .init();

    let addr = format!("{}:{}", base::config::CONFIG.host, base::config::CONFIG.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("GameServer listening on {}", addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handler::game_server::handle_game_server(socket, addr).await {
                tracing::error!("Error handling connection from {}: {}", addr, e);
            }
        });
    }
}
