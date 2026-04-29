const GIRL_EXP_TABLE: [u32; 81] = [
    0, 400, 600, 800, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000, 2100, 2200, 2300, 2400, 2500, 2600, 2700, 2800, 2900, 3000, 3100, 3200, 3300, 3400, 3500, 3500, 3600, 3600,
    3700, 3700, 3800, 3800, 3900, 3900, 4000, 4200, 4500, 4800, 5100, 5400, 5700, 6000, 6300, 6600, 7000, 8000, 8300, 8600, 9900, 10500, 11200, 11800, 12500, 13200, 13900, 14100, 14900, 15600, 16400,
    17200, 18000, 18800, 19600, 20500, 21300, 24000, 27000, 30000, 33000, 36000, 39000, 42000, 45000, 48000, 51000, 0,
];

pub fn calc_girl_exp(current_level: u32, current_exp: u32, add_exp: u32) -> (u32, u32) {
    let mut level = current_level;
    let mut exp = current_exp;
    let mut gain = add_exp;
    loop {
        if level >= 80 {
            return (80, 0);
        }
        let need = GIRL_EXP_TABLE[level as usize];
        if need == 0 {
            return (level, 0);
        }
        let remain = need.saturating_sub(exp);
        if gain >= remain {
            gain -= remain;
            level += 1;
            exp = 0;
        } else {
            exp += gain;
            return (level, exp);
        }
    }
}
