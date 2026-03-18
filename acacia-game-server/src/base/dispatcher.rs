use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::warn;

use super::conn::Conn;

#[derive(Debug, Default)]
pub enum HandlerResult {
    #[default]
    Ok,
}

type BoxFuture = Pin<Box<dyn Future<Output = anyhow::Result<HandlerResult>> + Send>>;
type HandlerFn = fn(Arc<Conn>, Vec<u8>) -> BoxFuture;

pub struct HandlerRegistration {
    pub msg_id: i32,
    pub func: HandlerFn,
    pub name: &'static str,
}

inventory::collect!(HandlerRegistration);

pub static DISPATCHER: std::sync::LazyLock<Dispatcher> = std::sync::LazyLock::new(Dispatcher::new);

pub struct Dispatcher {
    handlers: HashMap<i32, Vec<HandlerEntry>>,
}

struct HandlerEntry {
    func: HandlerFn,
    name: &'static str,
}

impl Dispatcher {
    fn new() -> Self {
        let mut handlers: HashMap<i32, Vec<HandlerEntry>> = HashMap::new();
        for reg in inventory::iter::<HandlerRegistration> {
            handlers.entry(reg.msg_id).or_default().push(HandlerEntry { func: reg.func, name: reg.name });
        }
        Self { handlers }
    }

    pub async fn dispatch(&self, conn: &Arc<Conn>, msg_id: i32, buf: &[u8]) -> anyhow::Result<HandlerResult> {
        if let Some(entries) = self.handlers.get(&msg_id) {
            for entry in entries {
                let result = (entry.func)(conn.clone(), buf.to_vec()).await?;
                match result {
                    HandlerResult::Ok => {}
                }
            }
            Ok(HandlerResult::Ok)
        } else {
            warn!("No handler for msg_id={}", msg_id);
            Ok(HandlerResult::Ok)
        }
    }
}
