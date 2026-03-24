use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use acacia_commons::bits::set_attribute;
use acacia_derive::func_reg;
use acacia_game_msg::protocol::{self, Pf};

#[func_reg(Pf::NtfSetattr)]
async fn ntf_setattr(conn: Arc<Conn>, msgdata: protocol::NtfSetAttr) -> anyhow::Result<()> {
    let key = set_attribute(msgdata.gid, msgdata.sid)?;
    let rsp = protocol::NtfSyncPlayer {
        custom: HashMap::<u32, u32>::from([(key, msgdata.val)]),
        ..protocol::NtfSyncPlayer::default()
    };
    conn.manager().await.attrs.set(key, msgdata.val).await?;
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;
    Ok(())
}
