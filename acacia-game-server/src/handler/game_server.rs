use tokio::net::TcpStream;
use tracing::info;

use crate::base::conn::{CONNS, Conn};

pub async fn handle_game_server(stream: TcpStream, addr: std::net::SocketAddr) -> anyhow::Result<()> {
    let (conn, reader) = Conn::new(stream, addr);
    CONNS.add(conn.clone());
    info!("Connecting to the Client {}", conn.addr);
    conn.recv_loop(reader).await;
    info!("Connection closed {}", conn.addr);
    CONNS.remove(&conn);
    Ok(())
}
