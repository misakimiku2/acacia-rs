#[derive(Debug, Clone, Copy)]
pub struct GirlRoomFurniture(u32);

impl GirlRoomFurniture {
    pub fn from_value(val: u32) -> Self {
        Self(val)
    }

    pub fn as_value(&self) -> u32 {
        self.0
    }

    pub fn get(&self, index: usize) -> u8 {
        assert!(index <= 9, "Index out of bounds (0-9)");
        let shift = index * 3;
        ((self.0 >> shift) & 0b111) as u8
    }

    pub fn set(&mut self, index: usize, value: u8) {
        assert!(index <= 9, "Index out of bounds (0-9)");
        let value = (value.min(7)) as u32;
        let shift = index * 3;
        let mask = 0b111 << shift;
        self.0 = (self.0 & !mask) | (value << shift);
    }

    pub fn add(&mut self, index: usize, value: u8) {
        let current = self.get(index);
        let new_value = current.saturating_add(value).min(7);
        self.set(index, new_value);
    }
}
