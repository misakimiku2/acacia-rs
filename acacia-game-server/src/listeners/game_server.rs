use tokio::net::TcpStream;
use tracing::info;

use crate::base::config::CONFIG;
use crate::base::conn::{CONNS, Conn};

pub async fn listen_game_server() {
    let addr = format!("{}:{}", CONFIG.host, CONFIG.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("GameServer failed to bind port");
    tracing::info!("GameServer listening on {}", addr);
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_game_server(socket, addr).await {
                        tracing::error!("GameServer error handling connection from {}: {}", addr, e);
                    }
                });
            }
            Err(e) => {
                tracing::error!("GameServer error accept: {}", e);
            }
        }
    }
}

async fn handle_game_server(stream: TcpStream, addr: std::net::SocketAddr) -> anyhow::Result<()> {
    let (conn, reader) = Conn::new(stream, addr);
    CONNS.add(conn.clone());
    info!("Connecting to the Client {}", conn.addr);
    conn.recv_loop(reader).await;
    info!("Connection closed {}", conn.addr);
    CONNS.remove(&conn);
    Ok(())
}
