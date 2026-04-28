use acacia_commons::lua_utils::lua_serde;
use acacia_data::tables::tables;
use anyhow::Context;
use rand::distr::{Distribution, weighted::WeightedIndex};
use rand::rng;
use tracing::warn;

pub fn drop(id: u32) -> anyhow::Result<Vec<((u16, u16, u16, u16), u32)>> {
    let drop_data = tables().drop.find_one("id_1", id).with_context(|| format!("The specified drop_data does not exist."))?;

    let mut result = Vec::new();
    for [grop_id, _sort, times] in &drop_data.drop {
        for _ in 0..*times {
            result.extend(drop_grop(*grop_id)?);
        }
    }
    Ok(result)
}

pub fn drop_grop(id: u32) -> anyhow::Result<Vec<((u16, u16, u16, u16), u32)>> {
    let drop_grop_data = tables().drop_grop.find_one("id_1", id).with_context(|| format!("The specified drop_data does not exist."))?;
    let rand_type = drop_grop_data.rand_type.unwrap_or(0);

    let mut result = Vec::new();
    match rand_type {
        0 => {
            let grop: Vec<((u16, u16, u16, u16), u32, u32, u32)> = lua_serde::from_str(&drop_grop_data.grop).map_err(anyhow::Error::msg)?;
            for (gdpl, _weight, num, _drop_effect_id) in grop {
                result.push((gdpl, num));
            }
        }
        1 => {
            let grop: Vec<((u16, u16, u16, u16), u32, u32, u32)> = lua_serde::from_str(&drop_grop_data.grop).map_err(anyhow::Error::msg)?;
            let weights: Vec<u32> = grop.iter().map(|(_, weight, _, _)| *weight).collect();
            let dist = WeightedIndex::new(&weights)?;
            let mut rng = rng();
            let index = dist.sample(&mut rng);
            let (gdpl, _, num, _) = grop[index];
            result.push((gdpl, num));
        }
        2 => {
            let group_grop: Vec<(u32, u32, Vec<((u16, u16, u16, u16), u32, u32, u32)>)> = lua_serde::from_str(&drop_grop_data.grop).map_err(anyhow::Error::msg)?;
            let group_weights: Vec<u32> = group_grop.iter().map(|(_, weight, _)| *weight).collect();
            let group_dist = WeightedIndex::new(&group_weights)?;
            let mut rng = rng();
            let group_index = group_dist.sample(&mut rng);
            let (_, _, grop) = &group_grop[group_index];

            let weights: Vec<u32> = grop.iter().map(|(_, weight, _, _)| *weight).collect();
            let dist = WeightedIndex::new(&weights)?;
            let index = dist.sample(&mut rng);
            let (gdpl, _, num, _) = grop[index];
            result.push((gdpl, num));
        }
        _ => {
            warn!("Unknwon rand_type: {}", rand_type)
        }
    }
    Ok(result)
}
