use acacia_database::manager::player::PlayerManager;

#[derive(Debug, Clone, Copy)]
pub struct GirlRoomFurniture(u32);

impl GirlRoomFurniture {
    pub fn from_value(val: u32) -> Self {
        Self(val)
    }

    pub fn as_value(&self) -> u32 {
        self.0
    }

    pub fn get(&self, index: u32) -> u32 {
        assert!(index <= 9, "Index out of bounds (0-9)");
        let shift = index * 3;
        (self.0 >> shift) & 0b111
    }

    pub fn set(&mut self, index: u32, value: u32) {
        assert!(index <= 9, "Index out of bounds (0-9)");
        let value = (value.min(7)) as u32;
        let shift = index * 3;
        let mask = 0b111 << shift;
        self.0 = (self.0 & !mask) | (value << shift);
    }

    pub fn add(&mut self, index: u32, value: u32) {
        let current = self.get(index);
        let new_value = current.saturating_add(value).min(7);
        self.set(index, new_value);
    }

    pub fn get_idx_by_gift_id(gift_id: u32) -> (u32, u32) {
        let furniture_id = gift_id % 100 + 1;
        let furniture_attr_idx = (furniture_id - 1) / 10;
        let furniture_sub_idx = (furniture_id - 1) % 10;
        return (furniture_attr_idx, furniture_sub_idx);
    }
}

pub struct GirlRoomFurnitureFold<'a> {
    manager: &'a PlayerManager,
    area_id: u32,
    value: u32,
}

impl<'a> GirlRoomFurnitureFold<'a> {
    pub async fn from(manager: &'a PlayerManager, area_id: u32) -> anyhow::Result<Self> {
        let value = manager.attrs.get_attr(101, area_id * 50 + 23).await?.1;
        Ok(Self {
            manager: manager,
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
        self.manager.attrs.set_attr(101, self.area_id * 50 + 23, self.value).await
    }

    pub async fn set_by_tmp_id(&mut self, tmp_id: u32, fold: bool) -> anyhow::Result<(u32, u32)> {
        self.set(tmp_id % 100, fold).await
    }

    pub fn toggle(&mut self, index: u32) {
        assert!(index <= 31, "Index out of bounds (0-31)");
        self.value ^= 1 << index;
    }
}
