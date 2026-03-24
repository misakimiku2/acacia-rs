use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::warn;

use crate::base::conn::Conn;

type BoxFuture = Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;
type CallGsHandlerFn = fn(Arc<Conn>, acacia_game_msg::protocol::ReqCallGs) -> BoxFuture;

pub struct CallGsHandlerRegistration {
    pub api_name: &'static str,
    pub func: CallGsHandlerFn,
    pub name: &'static str,
}

inventory::collect!(CallGsHandlerRegistration);

pub static CALLGS_DISPATCHER: std::sync::LazyLock<CallGsDispatcher> = std::sync::LazyLock::new(CallGsDispatcher::new);

pub struct CallGsDispatcher {
    handlers: HashMap<&'static str, CallGsHandlerEntry>,
}

struct CallGsHandlerEntry {
    func: CallGsHandlerFn,
    #[allow(dead_code)]
    name: &'static str,
}

impl CallGsDispatcher {
    fn new() -> Self {
        let mut handlers = HashMap::new();
        for reg in inventory::iter::<CallGsHandlerRegistration> {
            handlers.insert(reg.api_name, CallGsHandlerEntry { func: reg.func, name: reg.name });
        }
        Self { handlers }
    }

    pub async fn dispatch(&self, conn: &Arc<Conn>, msgdata: acacia_game_msg::protocol::ReqCallGs) -> anyhow::Result<()> {
        if let Some(entry) = self.handlers.get(msgdata.api.as_str()) {
            (entry.func)(conn.clone(), msgdata).await
        } else {
            warn!("No callgs handler for api={}", msgdata.api);
            Ok(())
        }
    }
}
