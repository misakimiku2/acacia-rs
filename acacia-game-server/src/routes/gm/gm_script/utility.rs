//! 本地私服增强 GM 命令（acacia-rs 本地化改造）
//! - GM.SetAttr: 通用玩家属性设置
//! - GM.PassGuide: 一键写全部关卡通过记录（解锁依赖"关卡进度"的全部功能）
//! - GM.MaxAllGirl: 全部角色卡与武器升至满级 (Lv.80)

use std::sync::Arc;

use acacia_commons::bits::get_template;
use acacia_data::tables::tables;
use acacia_game_msg::core;
use acacia_game_msg::protocol::Pf;

use crate::base::conn::Conn;

// ============ GM.SetAttr ============

#[derive(Debug)]
pub struct SetAttr {
    pub gid: u32,
    pub id: u32,
    pub value: u32,
}

impl SetAttr {
    /// 支持 `GM.SetAttr({22, 10124, 1})` 与 `GM.SetAttr(22, 10124, 1)` 两种写法
    pub fn from_args(args: &[String]) -> Self {
        let joined = args.join(",");
        let cleaned = joined.trim();
        let inner = cleaned
            .strip_prefix('{')
            .unwrap_or(cleaned)
            .strip_suffix('}')
            .unwrap_or(cleaned);
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        Self {
            gid: parts.first().and_then(|s| s.parse().ok()).unwrap_or(0),
            id: parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            value: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0),
        }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        me.set_attribute(self.gid, self.id, self.value).await?;
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        tracing::info!("GM.SetAttr: ({}, {}) = {}", self.gid, self.id, self.value);
        Ok(())
    }
}

// ============ GM.PassGuide ============

#[derive(Debug)]
pub struct PassGuide {}

impl PassGuide {
    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        let mut count = 0u32;
        // 遍历关卡表: 属性 22 = 通过次数, 属性 21 = 星级
        for lv in tables().chapter_level.all() {
            me.set_attribute(22, lv.id, 1).await?;
            me.set_attribute(21, lv.id, 1).await?;
            count += 1;
        }
        // 新手引导关卡满状态（参考 chapter_level_logger 的处理: 10121-10124 星级 7）
        for nid in [10121u32, 10122, 10123, 10124, 10101, 10102, 10103, 10104] {
            me.set_attribute(22, nid, 1).await?;
            me.set_attribute(21, nid, 7).await?;
        }
        tracing::info!("GM.PassGuide done: {} levels marked as passed", count);
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}

// ============ GM.MaxAllGirl ============

#[derive(Debug)]
pub struct MaxAllGirl {
    pub level: u32,
}

impl MaxAllGirl {
    pub fn from_args(args: &[String]) -> Self {
        let lv = args
            .iter()
            .map(|s| s.trim().trim_start_matches('{').trim_end_matches('}').trim())
            .find_map(|s| s.parse::<u32>().ok())
            .unwrap_or(80);
        Self { level: lv.clamp(1, 80) }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        let database = conn.database().await;
        let items = database.items.find_all().await?;
        let mut n = 0u32;
        for mut item in items {
            let (g, _d, _p, _l) = get_template(item.template);
            // genre 1 = 角色卡, genre 2 = 武器
            if g == 1 || g == 2 {
                let enhance = item
                    .enhance
                    .get_or_insert_with(|| core::Enhance { level: 1, ..Default::default() });
                if enhance.level < self.level {
                    enhance.level = self.level;
                    enhance.exp = 0;
                    me.set_item(item).await?;
                    n += 1;
                }
            }
        }
        tracing::info!("GM.MaxAllGirl done: {} items set to Lv.{}", n, self.level);
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}
