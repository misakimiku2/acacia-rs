use crate::*;

#[test]
fn test_data() -> anyhow::Result<()> {
    tables::load_tables()?;
    let data = tables::tables().gift_exchange.find_one("gift_id_1", 2306);
    println!("{:?}", data);
    let data = tables::tables().dorm_gift_template.find_one("gdpl", [8, 23, 11, 3]).unwrap();
    println!("{:?}", data);
    let data = tables::tables().gift.find_one("gift_id_1", data.param1);
    println!("{:?}", data);
    let data = tables::tables().chapter_level.find_one("id_1", 50174);
    println!("{:?}", data);
    println!("{:?}", tables::tables().battlepass_timelist.all());
    Ok(())
}
