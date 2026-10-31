//! 【本地私服增强】一键全进度 / 全奖励（acacia-rs 本地化改造）
//!
//! - `GM.PassAll`         全部主线 / 日常关卡通关满星，未通关的先补发首通奖励
//! - `GM.ClaimAll`        领取全部章节星级奖励（按当前星数逐档发放，幂等）
//! - `GM.CompleteAll`     PassAll + ClaimAll（启动器「全进度通关」按钮）
//! - `GM.House.UnlockAll` 宿舍全解锁（好感度 + 家具）
//!
//! 设计要点：
//! * 首通奖励必须在写入「通过次数」之前发放 —— `chapter_level_drop` 靠 gid22 == 0
//!   判定首通，先写 1 会把这批奖励永久吃掉，所以 PassAll 内部顺序固定为
//!   「发奖 → 写通过 → 写星级」。
//! * 剧情关（Type 2/7）只有 bit8 完成位，没有星级，不能计入章节星数，
//!   否则章节星级奖励会被虚高的星数一次全领完。

use std::sync::Arc;

use acacia_commons::lua_utils::lua_serde;
use acacia_data::tables::tables;
use acacia_game_msg::protocol::Pf;

use crate::base::conn::Conn;
use crate::logic::drop::drop;
use crate::logic::player::PlayerManager;

/// 属性组 4：引导完成度（sid 0 是总开关，5 = 全部完成）
const ATTR_GUIDE: u32 = 4;
/// 属性组 20：章节星级奖励领取位图（sid = (章节ID << 8) | 难度，bit N = 第 N 档）
const ATTR_CHAPTER_STAR_AWARD: u32 = 20;
/// 属性组 21：关卡状态（低 3 位星数，bit8 = 剧情关已完成）
const ATTR_LEVEL_STAR: u32 = 21;
/// 属性组 22：关卡通过次数
const ATTR_LEVEL_PASS: u32 = 22;
/// 属性组 101：宿舍数据（sid = 角色ID * 50 + 偏移）
const ATTR_HOUSE: u32 = 101;
/// 属性组 175：宿舍任务引导（sid = 任务/步骤 ID，值 1 = 已完成）
const ATTR_HOUSE_TASK_GUIDE: u32 = 175;

/// 三星
const LEVEL_STAR_FULL: u32 = 0b111;
/// 剧情关完成标记
const PLOT_DONE_FLAG: u32 = 1 << 8;
/// 首通奖励补发标记（GM 专用位，客户端只读低 3 位星数与 bit8，不认这一位）
const FIRST_CLEAR_GRANTED: u32 = 1 << 9;
/// 章节星级奖励最多 15 档（与 Chapter_GetStarAward 的位图约定一致）
const MAX_STAR_AWARD_INDEX: u32 = 15;
/// 家具组解锁掩码：10 组家具 × 每组 3 bit，全部选中第 1 件
const FURNITURE_UNLOCKED: u32 = 153_391_689;

/// 引导 ID 覆盖区间（与登录时 ensure_guide_bootstrap 保持一致）
const GUIDE_RANGES: [(u32, u32); 5] = [
    (1, 150),
    (10_000, 10_300),
    (11_000, 11_300),
    (12_000, 12_100),
    (22_000, 22_100),
];

/// 发放一组 drop id，返回发放条目数
async fn grant_drops(me: &mut PlayerManager, ids: Option<&[u32]>) -> anyhow::Result<u32> {
    let Some(ids) = ids else {
        return Ok(0);
    };
    let mut n = 0;
    for drop_id in ids {
        for ((g, d, p, l), num) in drop(*drop_id)? {
            me.add_item_count(g, d, p, l, num).await?;
            n += 1;
        }
    }
    Ok(n)
}

// ============ GM.PassAll ============

#[derive(Debug)]
pub struct PassAll {
    pub with_guide: bool,
}

impl PassAll {
    /// 默认连引导一起补满；`GM.PassAll(0)` 可只通关卡不动引导
    pub fn from_args(args: &[String]) -> Self {
        let with_guide = args
            .iter()
            .map(|s| s.trim().trim_start_matches('{').trim_end_matches('}').trim())
            .find_map(|s| s.parse::<u32>().ok())
            .map(|v| v != 0)
            .unwrap_or(true);
        Self { with_guide }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        let mut level_count = 0u32;
        let mut first_clear = 0u32;

        // 1. 新手引导全完成（宿舍等功能的开放判定依赖它）
        if self.with_guide {
            me.set_attribute(ATTR_GUIDE, 0, 5).await?;
            for (from, to) in GUIDE_RANGES {
                for guide_id in from..=to {
                    me.set_attribute(ATTR_GUIDE, guide_id, 999).await?;
                }
            }
        }

        // 2. 主线关卡：先补首通奖励，再写通过次数与星级
        for lv in tables().chapter_level.all() {
            let mut state = me.get_attribute(ATTR_LEVEL_STAR, lv.id).await?;
            let pass_times = me.get_attribute(ATTR_LEVEL_PASS, lv.id).await?;
            // 正常首通（pass == 0）或 GM 尚未补发过（bit9 未置）都要发一次首通奖励：
            // 之前跑过 GM.PassGuide 的存档 pass 已经是 1，仅靠 pass == 0 判定会永久漏发。
            if pass_times == 0 || state & FIRST_CLEAR_GRANTED == 0 {
                first_clear += grant_drops(&mut me, lv.first_drop_id.as_deref()).await?;
                if let Some(player_exp) = lv.player_exp {
                    me.add_exp(player_exp).await?;
                }
            }
            me.set_attribute(ATTR_LEVEL_PASS, lv.id, 1).await?;
            // 剧情关（Type 2/7）只记完成位并清掉遗留星数，战斗关给满星
            if matches!(lv.r#type, 2 | 7) {
                state = (state & !LEVEL_STAR_FULL) | PLOT_DONE_FLAG;
            } else {
                state |= LEVEL_STAR_FULL;
            }
            state |= FIRST_CLEAR_GRANTED;
            me.set_attribute(ATTR_LEVEL_STAR, lv.id, state).await?;
            level_count += 1;
        }

        // 3. 日常关卡（养成材料 / 金币本）：首通 + 满星 + bit8 已完成
        for lv in tables().daily_level.all() {
            let state = me.get_attribute(ATTR_LEVEL_STAR, lv.id).await?;
            let pass_times = me.get_attribute(ATTR_LEVEL_PASS, lv.id).await?;
            if pass_times == 0 || state & FIRST_CLEAR_GRANTED == 0 {
                first_clear += grant_drops(&mut me, lv.first_drop_id.as_deref()).await?;
                if let Some(player_exp) = lv.player_exp {
                    me.add_exp(player_exp).await?;
                }
            }
            me.set_attribute(ATTR_LEVEL_PASS, lv.id, 1).await?;
            me.set_attribute(ATTR_LEVEL_STAR, lv.id, state | LEVEL_STAR_FULL | PLOT_DONE_FLAG | FIRST_CLEAR_GRANTED).await?;
            level_count += 1;
        }

        tracing::info!("GM.PassAll done: {} levels marked passed, {} first-clear reward rows granted", level_count, first_clear);
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}

// ============ GM.ClaimAll ============

#[derive(Debug)]
pub struct ClaimAll {}

impl ClaimAll {
    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        let mut claimed_rows = 0u32;
        let mut claimed_tiers = 0u32;

        for chapter in tables().chapter_main.all() {
            // StarAward: {{星数, {{g,d,p,l,n},...}}, ...}
            let awards: Vec<(u32, Vec<Vec<u32>>)> = chapter
                .star_award
                .as_deref()
                .and_then(|s| lua_serde::from_str(s).ok())
                .unwrap_or_default();
            if awards.is_empty() {
                continue;
            }

            let attr_sid = (chapter.id << 8) | chapter.difficult;
            let mut mask = me.get_attribute(ATTR_CHAPTER_STAR_AWARD, attr_sid).await?;

            // 章节星数 = 章节内所有关卡星数之和（剧情关不计星）。
            // Level 列含嵌套 table 的章节会让整行解析失败（chapter.level 为空），
            // 这类章节退化为按全局星数判定，避免它们的星级奖励永远领不到。
            let mut star_count = 0u32;
            match chapter.level.as_deref() {
                Some(levels) if !levels.is_empty() => {
                    for level_id in levels {
                        star_count += (me.get_attribute(ATTR_LEVEL_STAR, *level_id).await? & LEVEL_STAR_FULL).count_ones();
                    }
                }
                _ => {
                    for lv in tables().chapter_level.all() {
                        star_count += (me.get_attribute(ATTR_LEVEL_STAR, lv.id).await? & LEVEL_STAR_FULL).count_ones();
                    }
                }
            }

            for (i, (required, rewards)) in awards.iter().enumerate() {
                let index = (i + 1) as u32;
                if index > MAX_STAR_AWARD_INDEX {
                    break;
                }
                let bit = 1u32 << index;
                if mask & bit != 0 || star_count < *required {
                    continue;
                }
                for row in rewards {
                    if row.len() >= 5 {
                        me.add_item_count(row[0] as u16, row[1] as u16, row[2] as u16, row[3] as u16, row[4]).await?;
                        claimed_rows += 1;
                    }
                }
                mask |= bit;
                claimed_tiers += 1;
            }
            me.set_attribute(ATTR_CHAPTER_STAR_AWARD, attr_sid, mask).await?;
        }

        tracing::info!("GM.ClaimAll done: {} star-award tiers, {} reward rows granted", claimed_tiers, claimed_rows);
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}

// ============ GM.CompleteAll ============

#[derive(Debug)]
pub struct CompleteAll {}

impl CompleteAll {
    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        PassAll { with_guide: true }.run(conn.clone()).await?;
        ClaimAll {}.run(conn.clone()).await?;
        Ok(())
    }
}

// ============ GM.House.UnlockAll ============

#[derive(Debug)]
pub struct HouseUnlockAll {}

impl HouseUnlockAll {
    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        let mut count = 0u32;

        // 1. 好感度拉满（组 101，sid = 角色ID * 50）
        for girl in tables().house_open_girl.all() {
            me.set_attribute(ATTR_HOUSE, girl.girl_id * 50, 500).await?;
            count += 1;
        }

        // 2. 角色房间家具：sid = 角色ID * 50 + 10..19
        for girl_id in 0u32..=50 {
            for offset in 10u32..=19 {
                me.set_attribute(ATTR_HOUSE, girl_id * 50 + offset, FURNITURE_UNLOCKED).await?;
                count += 1;
            }
        }

        // 3. 按摩房 / 温泉 / 沙滩 家具
        for (from, to) in [(10_010u32, 10_019u32), (15_001, 15_010), (17_101, 17_110)] {
            for sid in from..=to {
                me.set_attribute(ATTR_HOUSE, sid, FURNITURE_UNLOCKED).await?;
                count += 1;
            }
        }
        // 按摩房家具组选中状态
        me.set_attribute(ATTR_HOUSE, 10_020, 1).await?;

        tracing::info!("GM.House.UnlockAll done: {} house attrs written", count);
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}

// ============ GM.House.GiveAllFurniture ============

/// 发放全部宿舍礼物 / 家具（`item/templates/dorm_gift.txt`，Genre=8，324 条）。
///
/// 宿舍里的送礼类教程任务（例如「给里芙的礼物」要求送【嵌入式展示柜】）
/// 必须有对应家具在背包里才能推进，而这些模板原先不在 AddAllItem 的发放范围内。
#[derive(Debug)]
pub struct HouseGiveAllFurniture {
    pub count: u32,
}

impl HouseGiveAllFurniture {
    pub fn from_args(args: &[String]) -> Self {
        let count = args
            .iter()
            .map(|s| s.trim().trim_start_matches('{').trim_end_matches('}').trim())
            .find_map(|s| s.parse::<u32>().ok())
            .unwrap_or(99)
            .clamp(1, 9999);
        Self { count }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        let mut n = 0u32;
        for template_data in tables().dorm_gift_template.all() {
            me.set_item_count(template_data.genre, template_data.detail, template_data.particular, template_data.level, self.count)
                .await?;
            n += 1;
        }
        tracing::info!("GM.House.GiveAllFurniture done: {} furniture templates set to x{}", n, self.count);
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}

// ============ GM.House.SkipTaskGuide ============

/// 宿舍任务引导（左侧任务追踪里的「给里芙的礼物」这类教程任务）。
///
/// 这类任务完全由客户端本地配置驱动，服务端只知道一个 gid 175 的进度组：
/// 客户端拿登录时同步下来的 175 组判断每一步做没做。
/// 这里把 1..=N 全部写成 1（完成），让追踪面板不再弹这些教程任务。
#[derive(Debug)]
pub struct HouseSkipTaskGuide {
    pub max_sid: u32,
}

impl HouseSkipTaskGuide {
    pub fn from_args(args: &[String]) -> Self {
        let max_sid = args
            .iter()
            .map(|s| s.trim().trim_start_matches('{').trim_end_matches('}').trim())
            .find_map(|s| s.parse::<u32>().ok())
            .unwrap_or(30)
            .clamp(1, 200);
        Self { max_sid }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        for sid in 1..=self.max_sid {
            me.set_attribute(ATTR_HOUSE_TASK_GUIDE, sid, 1).await?;
        }
        tracing::info!("GM.House.SkipTaskGuide done: gid {} sid 1..={} set to 1", ATTR_HOUSE_TASK_GUIDE, self.max_sid);
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}
