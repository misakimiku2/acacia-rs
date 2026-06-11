use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Mutex, OnceLock};

use super::game_server_base::{GameServerEvent, GameServerMessage};
use super::game_server_process::GameServerProcess;

static GAME_SERVER_MANAGER: OnceLock<Mutex<GameServerManager>> = OnceLock::new();

pub fn game_server_manager() -> &'static Mutex<GameServerManager> {
    GAME_SERVER_MANAGER.get_or_init(|| Mutex::new(GameServerManager::new()))
}

pub struct GameServerManager {
    process: Option<GameServerProcess>,
    sender: Sender<GameServerMessage>,
    receiver: Receiver<GameServerMessage>,
}

impl GameServerManager {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        Self { process: None, sender, receiver }
    }

    pub fn start(&mut self) -> color_eyre::Result<()> {
        if self.is_running()? {
            return Ok(());
        }
        match GameServerProcess::spawn(self.sender.clone()) {
            Ok(process) => {
                self.process = Some(process);
                Ok(())
            }
            Err(error) => {
                let message = error.to_string();
                let _ = self.sender.send((GameServerEvent::InError, message.clone()));
                Err(error)
            }
        }
    }

    pub fn stop(&mut self) -> color_eyre::Result<()> {
        if let Some(mut process) = self.process.take() {
            process.stop()?;
        }

        Ok(())
    }

    pub fn _restart(&mut self) -> color_eyre::Result<()> {
        self.stop()?;
        self.start()?;

        Ok(())
    }

    pub fn is_running(&mut self) -> color_eyre::Result<bool> {
        let Some(process) = &mut self.process else {
            return Ok(false);
        };

        let running = process.is_running()?;

        if !running {
            self.process = None;
        }

        Ok(running)
    }

    pub fn try_recv(&self) -> Result<GameServerMessage, TryRecvError> {
        self.receiver.try_recv()
    }
}
