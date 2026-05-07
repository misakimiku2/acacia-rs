use std::sync::Arc;

use crate::{base::conn::Conn, logic::item::new_item};
use acacia_data::tables::tables;
use acacia_game_msg::protocol::Pf;

#[derive(Debug)]
pub struct AddAllItem {}

impl AddAllItem {
    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        for template_data in tables().card_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().weapon_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().support_card_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().card_skin_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().card_skin_parts_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().weapon_parts_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().weapon_skin_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().profile_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().manifestation_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().main_scene_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().ar_item_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().call_item_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            if me.get_item_count(g, d, p, l).await? != 0 {
                continue;
            }
            new_item(&mut me, g, d, p, l, 1).await?;
        }
        for template_data in tables().suplies_template.all() {
            let g = template_data.genre;
            let d = template_data.detail;
            let p = template_data.particular;
            let l = template_data.level;
            me.set_item_count(g, d, p, l, 9999999).await?;
        }
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}
