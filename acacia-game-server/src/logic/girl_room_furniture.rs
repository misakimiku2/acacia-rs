use anyhow::{Context, bail};

use acacia_data::tables::tables;
use acacia_database::repositories::player::PlayerRepository;

pub struct GirlRoomFurniture<'a> {
    database: &'a PlayerRepository,
    area_id: u32,
}

impl<'a> GirlRoomFurniture<'a> {
    pub fn new(database: &'a PlayerRepository, area_id: u32) -> Self {
        Self { database: database, area_id: area_id }
    }

    pub async fn get(&self, index: u32) -> anyhow::Result<u32> {
        let (group_idx, sub_idx) = GirlRoomFurniture::get_group_idx(index);
        let value = self.database.attrs.get_attr(101, self.area_id * 50 + group_idx + 10).await?.1;
        let shift = sub_idx * 3;
        Ok((value >> shift) & 0b111)
    }

    pub async fn set(&self, index: u32, tmp_value: u32) -> anyhow::Result<(u32, u32)> {
        let (group_idx, sub_idx) = GirlRoomFurniture::get_group_idx(index);
        let mut value = self.database.attrs.get_attr(101, self.area_id * 50 + group_idx + 10).await?.1;
        let tmp_value = (tmp_value.min(7)) as u32;
        let shift = sub_idx * 3;
        let mask = 0b111 << shift;
        value = (value & !mask) | (tmp_value << shift);
        Ok(self.database.attrs.set_attr(101, self.area_id * 50 + group_idx + 10, value).await?)
    }

    pub async fn set_by_tmp_id(&self, tmp_id: u32, tmp_value: u32) -> anyhow::Result<(u32, u32)> {
        let support_furniture_data = tables()
            .support_furniture
            .find_one("furniture_tmp_id_1", tmp_id)
            .with_context(|| format!("The specified support_furniture_data does not exist."))?;
        if support_furniture_data.area_id != self.area_id {
            bail!("Mismatched area_id: {} != {}", support_furniture_data.area_id, self.area_id)
        }
        self.set(support_furniture_data.index, tmp_value).await
    }

    pub fn get_group_idx(tmp_idx: u32) -> (u32, u32) {
        return (tmp_idx / 10, tmp_idx % 10);
    }
}

pub struct GirlRoomFurnitureFold<'a> {
    database: &'a PlayerRepository,
    area_id: u32,
    value: u32,
}

impl<'a> GirlRoomFurnitureFold<'a> {
    pub async fn from(database: &'a PlayerRepository, area_id: u32) -> anyhow::Result<Self> {
        let value = database.attrs.get_attr(101, area_id * 50 + 23).await?.1;
        Ok(Self {
            database: database,
            area_id: area_id,
            value: value,
        })
    }

    pub fn get(&self, index: u32) -> bool {
        assert!(index <= 31, "Index out of bounds (0-31)");
        ((self.value >> index) & 1) == 1
    }

    pub async fn set(&mut self, index: u32, fold: bool) -> anyhow::Result<(u32, u32)> {
        assert!(index <= 31, "Index out of bounds (0-31)");
        if fold {
            self.value |= 1 << index;
        } else {
            self.value &= !(1 << index);
        }
        self.database.attrs.set_attr(101, self.area_id * 50 + 23, self.value).await
    }

    pub fn toggle(&mut self, index: u32) {
        assert!(index <= 31, "Index out of bounds (0-31)");
        self.value ^= 1 << index;
    }

    async fn __set_one_by_tmp_id(&mut self, tmp_id: u32, fold: bool) -> anyhow::Result<()> {
        let support_furniture_data = tables()
            .support_furniture
            .find_one("furniture_tmp_id_1", tmp_id)
            .with_context(|| format!("The specified support_furniture_data does not exist."))?;
        if support_furniture_data.area_id != self.area_id {
            bail!("Mismatched area_id: {} != {}", support_furniture_data.area_id, self.area_id)
        }
        if fold {
            self.value |= 1 << support_furniture_data.index;
        } else {
            self.value &= !(1 << support_furniture_data.index);
        }
        Ok(())
    }

    pub async fn set_by_tmp_id(&mut self, tmp_id: u32, fold: bool) -> anyhow::Result<(u32, u32)> {
        self.__set_one_by_tmp_id(tmp_id, fold).await?;
        if !fold {
            for furniture_pos_data in tables().furniture_pos.find("area_id_1", self.area_id) {
                if !furniture_pos_data.furniture_tmp_id.contains(&tmp_id) {
                    continue;
                }
                for &fold_tmp_id in &furniture_pos_data.furniture_tmp_id {
                    if fold_tmp_id == tmp_id {
                        continue;
                    }
                    self.__set_one_by_tmp_id(fold_tmp_id, true).await?;
                }
            }
        }
        self.database.attrs.set_attr(101, self.area_id * 50 + 23, self.value).await
    }
}
