use crate::*;

#[test]
fn test_data() -> anyhow::Result<()> {
    tables::load_tables()?;
    let data = tables::tables().gift_exchange.find_one("gift_id_1", 2306);
    println!("{:?}", data);
    Ok(())
}
