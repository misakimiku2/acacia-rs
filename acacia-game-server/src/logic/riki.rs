use crate::logic::player::PlayerManager;
use acacia_data::tables::tables;
use tracing::warn;

pub fn get_task(n_id: u32) -> (u32, u32) {
    let n_index = n_id / 1000;
    let n_val = n_id % 1000;

    let n_task_id = (n_index - 1) * 30 + (n_val + 29) / 30;
    let n_bit = n_val % 30;

    (n_task_id, n_bit)
}

pub async fn riki_unlock_item(me: &mut PlayerManager, r#type: u32, g: u16, d: u16, p: u16, l: u16) -> anyhow::Result<()> {
    let condition = format!("{{{g},{d},{p},{l}}}");
    let Some(riki_data) = tables().riki.find_one("type_condition_1", (r#type, &condition)) else {
        warn!("The specified riki_data does not exist. condition={:?}", condition);
        return Ok(());
    };
    let (ntask_id, nbit) = get_task(riki_data.id);
    let mut nvalue = me.get_attribute(103, ntask_id).await?;
    if nvalue & (1 << nbit) == 0 {
        nvalue |= 1 << nbit;
        me.set_attribute(103, ntask_id, nvalue).await?;
        if r#type == 1 {
            me.set_show_attr(0, me.get_show_attr(0).await? + 1).await?;
        } else if r#type == 2 {
            me.set_show_attr(19, me.get_show_attr(19).await? + 1).await?;
        } else if r#type == 5 {
            me.set_show_attr(18, me.get_show_attr(18).await? + 1).await?;
        }
    }
    Ok(())
}
