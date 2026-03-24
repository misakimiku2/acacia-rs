use acacia_game_msg::core;

use crate::db::init_db_by_auth;
use crate::manager::player::PlayerManager;

#[tokio::test]
async fn test() {
    init_db_by_auth("http://127.0.0.1:8000", String::from("root"), String::from("root"))
        .await
        .expect("Failed to init test db");

    let player = PlayerManager::new(10001);
    test_player_item(&player).await.expect("test_player_item failed");
}

async fn test_player_item(player: &PlayerManager) -> anyhow::Result<()> {
    let item_1 = core::Item {
        id: 0,
        template: 50001,
        ..core::Item::default()
    };
    println!("item_1: {:?}", item_1);
    player.items.update(item_1).await?;

    let item_2 = core::Item {
        id: 0,
        template: 50002,
        ..core::Item::default()
    };
    println!("item_2: {:?}", item_2);
    player.items.update(item_2).await?;

    let items = player.items.get_all().await?;
    println!("{:?}", items);

    player.items.del_all().await?;

    let items = player.items.get_all().await?;
    println!("{:?}", items);
    Ok(())
}
