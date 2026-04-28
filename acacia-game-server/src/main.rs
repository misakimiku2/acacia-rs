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
    mod callgs {
        mod activity;
        mod battle_pass_logic;
        mod boss_logic;
        mod boss_pvp_logic;
        mod chapter;
        mod dlc_logic;
        mod gacha;
        mod general;
        mod girl_skin;
        mod girl_weapon_skin;
        mod guide_logic;
        mod house;
        mod lineup;
        mod player_setting;
        mod rouge3_d;
        mod shop_logic;
        mod vir_capture;
        mod vir_capture_level;
        mod weapon;
    }
    mod none;
    mod ntf_logout;
    mod ntf_setattr;
    mod ntf_setstrattr;
    mod req_callgs;
    mod req_login;
    mod req_reconnect;
    mod req_rename;
}
pub mod gm {
    pub mod gm_script {
        pub mod gm_script;
        pub mod gm_script_activity;
        pub mod gm_script_add;
        pub mod gm_script_gacha;
        pub mod gm_script_house;
        pub mod gm_script_riki;
        pub mod gm_script_un_lock_all_level;
        pub mod gm_script_un_lock_level;
    }
}
pub mod logic {
    pub mod girl_love_level;
    pub mod girl_room_furniture;
    pub mod manager;
    pub mod riki;
    pub mod chapter;
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
