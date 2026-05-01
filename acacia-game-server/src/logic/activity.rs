use acacia_data::{schemas::ActivityData, tables::tables};
use tracing::warn;

use crate::{base::time::ServerTime, logic::player::PlayerManager};

pub fn activity_is_open(activity_data: &ActivityData) -> bool {
    let current_time = ServerTime::now();
    let start_time = activity_data.start_time.unwrap_or(0);
    if start_time > current_time {
        return false;
    }
    if let Some(end_time) = activity_data.end_time {
        if end_time < current_time {
            return false;
        }
    }
    return true;
}

pub async fn get_activity_diy_data(me: &PlayerManager, id: u32, task_id: u32) -> anyhow::Result<u32> {
    me.get_attribute(50, id * 10 + task_id).await
}

pub async fn set_activity_diy_data(me: &mut PlayerManager, id: u32, task_id: u32, value: u32) -> anyhow::Result<()> {
    me.set_attribute(50, id * 10 + task_id, value).await
}

pub fn get_activity_sign_max_days(activity_id: u32) -> u32 {
    let datas = tables().activity_sign_day.find("activity_id_1", activity_id);
    if !datas.is_empty() {
        return ServerTime::month_day_num();
    };
    let datas = tables().activity_sign_short.find("activity_id_1", activity_id);
    if !datas.is_empty() {
        return datas.iter().count() as u32;
    };
    warn!("Unknown signin_monthly activity_id: {:?}", activity_id);
    0
}
