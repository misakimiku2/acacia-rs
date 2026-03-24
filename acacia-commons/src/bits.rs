pub fn get_bits(n: u64, begin: u32, end: u32) -> u64 {
    let (begin, end) = if begin <= end { (begin, end) } else { (end, begin) };
    let width = end - begin + 1;
    let mask = if width == 64 { u64::MAX } else { (1u64 << width) - 1 };
    (n >> begin) & mask
}

pub fn set_bits(n: u64, begin: u32, end: u32) -> anyhow::Result<u64> {
    let (begin, end) = if begin <= end { (begin, end) } else { (end, begin) };
    let width = end - begin + 1;
    let max = if width == 64 { u64::MAX } else { 1u64 << width };
    if n >= max {
        anyhow::bail!("Out of {}bit range", width);
    }
    Ok(n << begin)
}

pub fn get_template(raw: u64) -> (u64, u64, u64, u64) {
    (get_bits(raw, 0, 15), get_bits(raw, 16, 31), get_bits(raw, 32, 47), get_bits(raw, 48, 63))
}

pub fn set_template(g: u16, d: u16, p: u16, l: u16) -> anyhow::Result<u64> {
    Ok(set_bits(g as u64, 0, 15)? + set_bits(d as u64, 16, 31)? + set_bits(p as u64, 32, 47)? + set_bits(l as u64, 48, 63)?)
}

pub fn get_attribute(raw: u64) -> (u64, u64) {
    (get_bits(raw, 16, 31), get_bits(raw, 0, 15))
}

pub fn set_attribute(group: u32, id: u32) -> anyhow::Result<u32> {
    Ok((set_bits(group as u64, 16, 31)? + set_bits(id as u64, 0, 15)?) as u32)
}
