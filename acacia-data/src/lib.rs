use acacia_game_msg::core::Player;
use prost::Message;
use std::sync::OnceLock;

static DATA: OnceLock<Player> = OnceLock::new();

pub fn data() -> &'static Player {
    DATA.get().expect("Failed to use data")
}

pub async fn init_data() -> anyhow::Result<()> {
    let data = Player::decode(&include_bytes!("../data/data.pb")[..])?;
    DATA.set(data).expect("Failed to initialize data");
    Ok(())
}
