mod base {
    pub mod dispatcher {
        mod callgs_handler;
        mod msg_handler;
        pub use callgs_handler::{CALLGS_DISPATCHER, CallGsHandlerRegistration};
        pub use msg_handler::{DISPATCHER, HandlerRegistration};
    }
    pub mod config;
    pub mod conn;
    pub mod packet;
}
mod handler {
    pub mod game_server;
}
mod func {
    mod callgs {
        mod girl;
    }
    mod none;
    mod ntf_logout;
    mod ntf_setattr;
    mod req_callgs;
    mod req_login;
    mod req_reconnect;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .compact()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&base::config::CONFIG.log)))
        .init();

    acacia_data::init_data().await?;

    if base::config::CONFIG.database.mode == "http" {
        acacia_database::db::init_db_by_auth(
            &base::config::CONFIG.database.address,
            base::config::CONFIG.database.username.clone(),
            base::config::CONFIG.database.password.clone(),
        )
        .await?;
    } else {
        acacia_database::db::init_db(&base::config::CONFIG.database.local).await?;
    }

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
