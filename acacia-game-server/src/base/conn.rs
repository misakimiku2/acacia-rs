use dashmap::DashMap;
use prost::Message;
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, error};

use super::dispatcher::DISPATCHER;
use super::packet::{PacketBuilder, PacketParser};
use crate::logic::player::PlayerManager;
use acacia_database::repositories::player::PlayerRepository;
use acacia_game_msg::{protocol, protocol::Pf};

static CONN_ID_SEQ: AtomicU64 = AtomicU64::new(1);
pub static CONNS: std::sync::LazyLock<Conns> = std::sync::LazyLock::new(Conns::new);

pub struct Conn {
    pub id: u64,
    pub addr: std::net::SocketAddr,
    writer: Mutex<tokio::net::tcp::OwnedWriteHalf>,
    uid: Mutex<Option<u64>>,
}

impl Conn {
    pub fn new(stream: TcpStream, addr: std::net::SocketAddr) -> (Arc<Self>, tokio::net::tcp::OwnedReadHalf) {
        let id = CONN_ID_SEQ.fetch_add(1, Ordering::Relaxed);
        let (read_half, write_half) = stream.into_split();
        let conn = Arc::new(Self {
            id,
            addr,
            writer: Mutex::new(write_half),
            uid: Mutex::new(None),
        });
        (conn, read_half)
    }

    pub fn uid(&self) -> Option<u64> {
        self.uid.try_lock().ok().and_then(|g| *g)
    }

    pub async fn bind_uid(self: &Arc<Self>, uid: u64) {
        {
            let mut g = self.uid.lock().await;
            if let Some(old_uid) = *g {
                CONNS.uid_map.remove(&old_uid);
            }
            *g = Some(uid);
            CONNS.uid_map.insert(uid, self.clone());
        }
    }

    pub async fn database(&self) -> PlayerRepository {
        let uid = *self.uid.lock().await;
        PlayerRepository::new(uid.expect("conn.database() called before uid was bound"))
    }

    pub async fn manager(self: &Arc<Self>) -> PlayerManager {
        PlayerManager::new(self.clone()).await
    }

    pub async fn recv_loop(self: &Arc<Self>, mut reader: tokio::net::tcp::OwnedReadHalf) {
        let mut parser = PacketParser::new();
        let mut buf = [0u8; 4096];

        loop {
            match reader.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    parser.add_raw(&buf[..n]);
                    while let Some(packet) = parser.parse_packet() {
                        let conn = self.clone();
                        let body = packet.body;
                        let msg_id = packet.msg_id as i32;
                        let msg_desc = Pf::try_from(msg_id).map(|pf| format!("{:?}", pf)).unwrap_or_else(|_| format!("Pf({})", msg_id));

                        tokio::spawn(async move {
                            if let Err(e) = DISPATCHER.dispatch(&conn, msg_id, &body).await {
                                error!("msgid {} dispatch error: {}", msg_desc, e);
                            }
                        });
                    }
                }
                Err(e) => {
                    error!("read error {}: {}", self.addr, e);
                    break;
                }
            }
        }
    }

    pub async fn send_msg<M: Message>(&self, msg_id: Pf, msg: &M, seq: u64) -> anyhow::Result<()> {
        let body = msg.encode_to_vec();
        debug!("<- {:?} {:?}", msg_id, msg);

        let packet = PacketBuilder::build(msg_id as u32, &body, seq);
        let mut writer = self.writer.lock().await;
        writer.write_all(&packet).await?;
        writer.flush().await?;
        Ok(())
    }

    pub async fn send_err(&self, err: impl Into<String>) -> anyhow::Result<()> {
        let rsp = protocol::NtfCallScript {
            api: "DefendLogic_SelectDiff".to_string(),
            arg: json!({
                "sErr": err.into()
            })
            .to_string(),
            ..protocol::NtfCallScript::default()
        };
        self.send_msg(Pf::NtfScript, &rsp, 0).await?;
        Ok(())
    }
}

impl std::fmt::Display for Conn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conn(id={}, addr={})", self.id, self.addr)
    }
}

pub struct Conns {
    map: DashMap<u64, Arc<Conn>>,
    uid_map: DashMap<u64, Arc<Conn>>,
}

impl Conns {
    fn new() -> Self {
        Self {
            map: DashMap::new(),
            uid_map: DashMap::new(),
        }
    }

    pub fn add(&self, conn: Arc<Conn>) {
        self.map.insert(conn.id, conn);
    }

    pub fn remove(&self, conn: &Conn) {
        self.map.remove(&conn.id);
        if let Some(uid) = conn.uid() {
            self.uid_map.remove(&uid);
        }
    }

    pub fn get_by_uid(&self, uid: u64) -> Option<Arc<Conn>> {
        self.uid_map.get(&uid).map(|r| r.value().clone())
    }

    pub fn count(&self) -> usize {
        self.map.len()
    }

    pub fn all(&self) -> Vec<Arc<Conn>> {
        self.map.iter().map(|r| r.value().clone()).collect()
    }
}
