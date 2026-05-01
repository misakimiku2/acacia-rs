use num_traits::{PrimInt, Unsigned};
use std::fmt::Debug;

pub fn get_bit<T, I>(val: T, idx: I) -> bool
where
    T: PrimInt + Unsigned,
    I: TryInto<usize>,
    I::Error: Debug,
{
    let idx = idx.try_into().unwrap();
    let mask = T::one() << idx;
    val & mask != T::zero()
}

pub fn set_bit<T, I>(val: T, idx: I, enable: bool) -> T
where
    T: PrimInt + Unsigned,
    I: TryInto<usize>,
    I::Error: Debug,
{
    let idx = idx.try_into().unwrap();
    let mask = T::one() << idx;
    if enable { val | mask } else { val & !mask }
}

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

pub fn get_template(raw: u64) -> (u16, u16, u16, u16) {
    (get_bits(raw, 0, 15) as u16, get_bits(raw, 16, 31) as u16, get_bits(raw, 32, 47) as u16, get_bits(raw, 48, 63) as u16)
}

pub fn set_template(g: u16, d: u16, p: u16, l: u16) -> u64 {
    (g as u64) | ((d as u64) << 16) | ((p as u64) << 32) | ((l as u64) << 48)
}

pub fn get_attribute(raw: u64) -> (u64, u64) {
    (get_bits(raw, 16, 31), get_bits(raw, 0, 15))
}

pub fn set_attribute(group: u32, id: u32) -> u32 {
    ((group & 0xffff) << 16) | (id & 0xffff)
}
