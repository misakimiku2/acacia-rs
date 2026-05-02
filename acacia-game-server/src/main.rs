use crate::base::config::CONFIG;

mod base {
    pub mod dispatcher {
        mod callgs_handler;
        mod msg_handler;
        pub use callgs_handler::{CALLGS_DISPATCHER, CallGsHandlerRegistration};
        pub use msg_handler::{DISPATCHER, HandlerRegistration};
    }
    pub mod config;
    pub mod conn;
    pub mod packet;
    pub mod time;
}
pub mod logic {
    pub mod activity;
    pub mod chapter;
    pub mod drop;
    pub mod girl;
    pub mod girl_love_level;
    pub mod girl_room_furniture;
    pub mod girl_spine;
    pub mod item;
    pub mod player;
    pub mod riki;
    pub mod weapon;
}
mod listeners {
    pub mod api_server;
    pub mod game_server;
    pub mod gm_server;
}
mod handlers {
    mod callgs;
    mod msg;
}
mod routes {
    pub mod api;
    pub mod gm;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .compact()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&base::config::CONFIG.log)))
        .init();

    acacia_data::init_data::load_init_data()?;
    acacia_data::tables::load_tables()?;

    if base::config::CONFIG.database.mode == "remote" {
        acacia_database::db::init_db_by_auth(
            &base::config::CONFIG.database.address,
            base::config::CONFIG.database.username.clone(),
            base::config::CONFIG.database.password.clone(),
        )
        .await?;
    } else {
        acacia_database::db::init_db("surrealkv://data/database").await?;
    }

    let mut tasks = tokio::task::JoinSet::new();
    tasks.spawn(async { listeners::game_server::listen_game_server().await });
    tasks.spawn(async { listeners::api_server::listen_api_server().await });
    if CONFIG.gm_server.enable {
        tasks.spawn(async { listeners::gm_server::listen_gm_server().await });
    } else {
        tracing::info!("GMServer disabled");
    }
    if let Some(ret) = tasks.join_next().await {
        tracing::error!("A server unexpectedly exited: {:?}", ret);
    }
    Ok(())
}
