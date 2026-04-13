use acacia_game_msg::core;

use crate::db::init_db_by_auth;
use crate::repositories::player::PlayerRepository;

#[tokio::test]
async fn test() {
    init_db_by_auth("http://127.0.0.1:8000", String::from("root"), String::from("root"))
        .await
        .expect("Failed to init test db");

    let player = PlayerRepository::new(10001);
    test_player_item(&player).await.expect("test_player_item failed");
}

async fn test_player_item(player: &PlayerRepository) -> anyhow::Result<()> {
    let item_1 = core::Item {
        id: 0,
        template: 50001,
        ..core::Item::default()
    };
    println!("item_1: {:?}", item_1);
    player.items.update_one(item_1).await?;

    let item_2 = core::Item {
        id: 0,
        template: 50002,
        ..core::Item::default()
    };
    println!("item_2: {:?}", item_2);
    player.items.update_one(item_2).await?;

    let items = player.items.find_all().await?;
    println!("{:?}", items);

    player.items.delete_all().await?;

    let items = player.items.find_all().await?;
    println!("{:?}", items);
    Ok(())
}
