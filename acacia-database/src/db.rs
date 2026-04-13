use std::sync::OnceLock;
use surrealdb::Surreal;
use surrealdb::engine::any::{Any, IntoEndpoint, connect};
use surrealdb::opt::auth::Root;

static DB: OnceLock<Surreal<Any>> = OnceLock::new();

pub fn db() -> &'static Surreal<Any> {
    DB.get().expect("Failed to use database")
}

async fn init_db_schema() -> surrealdb::Result<()> {
    db().use_ns("acacia_rs").use_db("acacia_rs").await?;
    let schema_player_item = include_str!("../migrations/0003_player_item.surql");
    db().query(schema_player_item).await?;
    let schema_player_solution = include_str!("../migrations/0004_player_solution.surql");
    db().query(schema_player_solution).await?;
    let schema_player_attr = include_str!("../migrations/0005_player_attr.surql");
    db().query(schema_player_attr).await?;
    let schema_player_str_attr = include_str!("../migrations/0006_player_str_attr.surql");
    db().query(schema_player_str_attr).await?;
    Ok(())
}

pub async fn init_db(address: impl IntoEndpoint) -> surrealdb::Result<()> {
    let database = connect(address).await?; // "surrealkv://data/acacia_rs.db"
    DB.set(database).expect("Failed to initialize database");
    init_db_schema().await?;
    Ok(())
}

pub async fn init_db_by_auth(address: impl IntoEndpoint, username: String, password: String) -> surrealdb::Result<()> {
    let database = connect(address).await?; // "http://127.0.0.1:8000"
    DB.set(database).expect("Failed to initialize database");
    db().signin(Root {
        username: username,
        password: password,
    })
    .await?;
    init_db_schema().await?;
    Ok(())
}
