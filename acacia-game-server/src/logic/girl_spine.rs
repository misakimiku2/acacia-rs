use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::logic::player::PlayerManager;
use acacia_commons::bits::get_template;
use acacia_data::tables::tables;
use acacia_game_msg::core;

#[derive(Debug, Default)]
pub struct SpineConfigs {
    configs: Vec<Option<SpineConfig>>,
    gdpl: (u16, u16, u16, u16),
}

impl SpineConfigs {
    pub async fn from(me: &PlayerManager, girl: &core::Item) -> anyhow::Result<Self> {
        let girl_gdpl = get_template(girl.template);
        let mut configs: Vec<Option<SpineConfig>> = match me.get_str_attribute(30, girl_gdpl.1 as u32).await?.trim() {
            "" => Vec::new(),
            s => serde_json::from_str(s)?,
        };
        configs.resize_with(girl_gdpl.2 as usize, || None);
        Ok(Self { configs, gdpl: girl_gdpl })
    }

    pub fn get_config(&mut self) -> &mut SpineConfig {
        let index = self.gdpl.2 as usize - 1;
        self.configs[index].get_or_insert_with(SpineConfig::default)
    }

    pub fn to_string(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(&self.configs)?)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SpineConfig {
    pub ns: u64,
    pub tbn: (u32, u32),
    pub tbr: Vec<i32>,
}

fn get_node(value: u64, sub_idx: usize) -> bool {
    value & (1 << sub_idx - 1) != 0
}

fn set_node(value: u64, sub_idx: usize, enable: bool) -> u64 {
    if enable { value | (1 << sub_idx - 1) } else { value & !(1 << sub_idx - 1) }
}

fn get_spine_node(spines: &Vec<u64>, mast_idx: usize, sub_idx: usize) -> bool {
    get_node(spines[mast_idx - 1], sub_idx)
}

fn set_spine_node(spines: &mut Vec<u64>, mast_idx: usize, sub_idx: usize, enable: bool) -> () {
    spines.resize(6, 0);
    spines[mast_idx - 1] = set_node(spines[mast_idx - 1], sub_idx, enable);
}

pub fn get_girl_spine_max_count(girl: &core::Item) -> usize {
    girl.enhance.as_ref().map_or(0, |enhance| enhance.spines.iter().filter(|&&x| get_node(x, 9)).count())
}

fn get_girl_spine_node_materials(girl: &core::Item, sub_idx: usize) -> anyhow::Result<HashMap<(u16, u16, u16, u16), u32>> {
    let girl_gdpl = get_template(girl.template);
    let Some(card_template_data) = tables().card_template.find_one("gdpl", [girl_gdpl.0, girl_gdpl.1, girl_gdpl.2, girl_gdpl.3]) else {
        bail!("The specified card_template_data does not exist. gdpl={:?}", girl_gdpl);
    };
    let Some(item_spine_data) = tables().item_spine.find_one("id_1", card_template_data.spine_id) else {
        bail!("The specified item_spine_data does not exist. id={}", card_template_data.spine_id);
    };
    let max_count = get_girl_spine_max_count(girl);
    let node_req = match max_count {
        0 => item_spine_data.node1_req,
        1 => item_spine_data.node2_req,
        2 => item_spine_data.node3_req,
        3 => item_spine_data.node4_req,
        4 => item_spine_data.node5_req,
        5 => item_spine_data.node6_req,
        _ => {
            bail!("Invalid girl spine max_count: {}", max_count);
        }
    };
    let Some(item_node_condition_data) = tables().item_node_condition.find_one("id_1", node_req) else {
        bail!("The specified item_node_condition_data does not exist. id={}", node_req);
    };
    let node_cost = match sub_idx {
        1 => &item_node_condition_data.node1_cost,
        2 => &item_node_condition_data.node2_cost,
        3 => &item_node_condition_data.node3_cost,
        4 => &item_node_condition_data.node4_cost,
        5 => &item_node_condition_data.node5_cost,
        6 => &item_node_condition_data.node6_cost,
        7 => &item_node_condition_data.node7_cost,
        8 => &item_node_condition_data.node8_cost,
        9 => &item_node_condition_data.node9_cost,
        _ => {
            bail!("Invalid girl spine node sub_idx: {}", sub_idx);
        }
    };
    let mut result = HashMap::<(u16, u16, u16, u16), u32>::new();
    for (g, d, p, l, n) in node_cost {
        result.insert((*g, *d, *p, *l), *n);
    }
    Ok(result)
}

pub async fn girl_spine_unlock(girl: &mut core::Item, mast_idx: usize, sub_idx: usize) -> anyhow::Result<HashMap<(u16, u16, u16, u16), u32>> {
    if let Some(enhance) = girl.enhance.as_ref() {
        if !enhance.spines.iter().enumerate().all(|(i, &v)| i == (mast_idx - 1) || v == 0 || get_node(v, 9) == true) {
            bail!("Invalid girl spine node mast_idx: {} of spines {:?}", mast_idx, enhance.spines);
        };
        if sub_idx != 1 && !get_spine_node(&enhance.spines, mast_idx, sub_idx - 1) {
            bail!("Invalid girl spine node mast_idx: {} of spines {:?}", mast_idx, enhance.spines);
        };
    }
    let mat = get_girl_spine_node_materials(girl, sub_idx)?;
    let enhance: &mut core::Enhance = girl.enhance.get_or_insert_with(|| core::Enhance { level: 1, ..Default::default() });
    set_spine_node(&mut enhance.spines, mast_idx, sub_idx, true);
    Ok(mat)
}
