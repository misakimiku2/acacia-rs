use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use acacia_commons::bits::set_attribute;
use acacia_derive::func_reg;
use acacia_game_msg::protocol::{self, Pf};

#[func_reg(Pf::NtfSetstrattr)]
async fn ntf_setstrattr(conn: Arc<Conn>, msgdata: protocol::NtfSetStrAttr) -> anyhow::Result<()> {
    let key = set_attribute(msgdata.gid, msgdata.sid);
    conn.database().await.str_attrs.update_one((key, msgdata.val.clone())).await?;
    let rsp = protocol::NtfSyncPlayer {
        custom_str: HashMap::<u32, String>::from([(key, msgdata.val)]),
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;
    Ok(())
}
