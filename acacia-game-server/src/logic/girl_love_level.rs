pub const MAX_LEVEL: u32 = 50;
pub const MAX_TOTAL_EXP: u32 = 140000;

pub fn get_exp_limit_for_level(level: u32) -> u32 {
    match level {
        0 => 0,
        1 => 800,
        2..=9 => 900,
        10 => 1800,
        11..=20 => 2020,
        21..=30 => 3000,
        31..=40 => 3600,
        41..=48 => 4800,
        49 => 5600,
        _ => 0,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GirlLoveLevel {
    total_exp: u32,
}

impl GirlLoveLevel {
    pub const MAX_LEVEL: u32 = 50;

    pub fn from_value(attr: u32) -> Self {
        if attr == 0 {
            return Self { total_exp: 0 };
        }
        let level = attr & 0x7FF;
        let current_exp = (attr >> 11) & 0xFFFFF;

        if level >= Self::MAX_LEVEL {
            return Self { total_exp: MAX_TOTAL_EXP };
        }

        let mut total = 0;
        for lv in 1..level {
            total += get_exp_limit_for_level(lv);
        }
        total += current_exp;

        let max_exp = MAX_TOTAL_EXP;
        if total > max_exp {
            total = max_exp;
        }

        Self { total_exp: total }
    }

    pub fn as_value(&self) -> u32 {
        let max_exp = MAX_TOTAL_EXP;
        if self.total_exp >= max_exp {
            return (Self::MAX_LEVEL & 0x7FF) | (0 << 11);
        }

        let mut temp_exp = self.total_exp;
        let mut level = 1;

        while level < Self::MAX_LEVEL {
            let req = get_exp_limit_for_level(level);
            if temp_exp >= req {
                temp_exp -= req;
                level += 1;
            } else {
                break;
            }
        }

        let current_exp = if level >= Self::MAX_LEVEL { 0 } else { temp_exp };

        (level & 0x7FF) | ((current_exp & 0xFFFFF) << 11)
    }

    pub fn add_exp(&mut self, amount: u32) {
        self.total_exp += amount;
        let max = MAX_TOTAL_EXP;
        if self.total_exp > max {
            self.total_exp = max;
        }
    }

    pub fn get_total_exp(&self) -> u32 {
        self.total_exp
    }

    pub fn get_level(&self) -> u32 {
        let attr = self.as_value();
        attr & 0x7FF
    }

    pub fn get_exp(&self) -> u32 {
        let attr = self.as_value();
        attr >> 11
    }
}
