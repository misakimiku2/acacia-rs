use acacia_game_msg::core::Player;
use prost::Message;
use std::sync::OnceLock;

static INIT_DATA: OnceLock<Player> = OnceLock::new();

pub fn init_data() -> &'static Player {
    INIT_DATA.get().expect("Failed to use init_data")
}

pub fn load_init_data() -> anyhow::Result<()> {
    let init_data = Player::decode(&include_bytes!("../data/init_data.pb")[..])?;
    INIT_DATA.set(init_data).expect("Failed to load init_data");
    Ok(())
}
