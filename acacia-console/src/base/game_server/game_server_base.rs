use color_eyre::eyre::eyre;
use std::path::PathBuf;

const GAMESERVER_EXE: &str = "acacia-game-server.exe";

pub fn game_server_path() -> color_eyre::Result<PathBuf> {
    let current_exe = std::env::current_exe()?;
    let exe_dir = current_exe.parent().ok_or_else(|| eyre!("Cannot read the directory where the current program is located"))?;
    Ok(exe_dir.join(GAMESERVER_EXE))
}

pub type GameServerMessage = (GameServerEvent, String);

#[derive(Clone, Copy)]
pub enum GameServerEvent {
    Debug,
    Info,
    Warn,
    Error,
    StdError,
    InError,
    Line,
}
