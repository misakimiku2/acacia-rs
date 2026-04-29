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
mod listeners {
    pub mod game_server;
    pub mod gm_server;
}
mod func {
    mod callgs;
    mod msg;
    pub mod gm {
        pub mod gm_script;
    }
}
pub mod logic {
    pub mod chapter;
    pub mod drop;
    pub mod girl;
    pub mod girl_love_level;
    pub mod girl_room_furniture;
    pub mod player;
    pub mod riki;
    pub mod weapon;
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

    let game_server_task = tokio::spawn(listeners::game_server::listen_game_server());
    let gm_server_task = tokio::spawn(listeners::gm_server::listen_gm_server());
    tokio::select! {
        ret = game_server_task => {
            tracing::error!("GameServer unexpectedly exited: {:?}", ret);
        }
        ret = gm_server_task => {
            tracing::error!("GMServer unexpectedly exited: {:?}", ret);
        }
    }
    Ok(())
}
