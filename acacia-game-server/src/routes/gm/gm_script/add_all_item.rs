use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_data::tables::tables;
use acacia_game_msg::protocol::Pf;

#[derive(Debug)]
pub struct AddAllItem {}

impl AddAllItem {
    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        for suplies_template_data in tables().suplies_template.all() {
            me.set_item_count(
                suplies_template_data.genre,
                suplies_template_data.detail,
                suplies_template_data.particular,
                suplies_template_data.level,
                9999999,
            )
            .await?;
        }
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}
