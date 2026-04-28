use serde::de;

pub fn parse_gdpl<E>(s: &str) -> Result<(u16, u16, u16, u16), E>
where
    E: de::Error,
{
    if s.trim().is_empty() {
        return Err(E::custom("expected 4 values, got empty string"));
    }

    let values: Vec<u16> = s
        .split(',')
        .map(|part| {
            let n = part.trim().parse::<u32>().map_err(E::custom)?;

            u16::try_from(n).map_err(E::custom)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let arr: [u16; 4] = values.try_into().map_err(|v: Vec<u16>| E::custom(format!("expected 4 values, got {}", v.len())))?;

    Ok(arr.into())
}
