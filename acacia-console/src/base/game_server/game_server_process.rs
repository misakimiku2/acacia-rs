use std::{
    io::{BufRead, BufReader},
    process::{Child, ChildStderr, ChildStdout, Command, Stdio},
    sync::mpsc::Sender,
    thread,
};

use crate::base::utils::strip_ansi;

use super::game_server_base::{GameServerEvent, GameServerMessage, game_server_path};

pub struct GameServerProcess {
    child: Child,
}

impl GameServerProcess {
    pub fn spawn(sender: Sender<GameServerMessage>) -> color_eyre::Result<Self> {
        let game_server_path = game_server_path()?;
        let mut child = Command::new(game_server_path).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        if let Some(stdout) = child.stdout.take() {
            Self::spawn_stdout_reader(sender.clone(), stdout);
        };
        if let Some(stderr) = child.stderr.take() {
            Self::spawn_stderr_reader(sender.clone(), stderr);
        };
        Ok(Self { child })
    }

    const STDOUT_PREFIX: [(&str, GameServerEvent); 4] = [
        ("ERROR ", GameServerEvent::Error),
        (" INFO ", GameServerEvent::Info),
        ("DEBUG ", GameServerEvent::Debug),
        (" WARN ", GameServerEvent::Warn),
    ];

    fn stdout_to_event(body: String) -> GameServerMessage {
        let plain_body = strip_ansi(&body);
        for (prefix, level) in Self::STDOUT_PREFIX {
            if let Some(plain_body) = plain_body.strip_prefix(prefix) {
                return (level, plain_body.to_owned());
            }
        }
        (GameServerEvent::Line, body)
    }

    fn spawn_stdout_reader(sender: Sender<GameServerMessage>, stdout: ChildStdout) {
        thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                match line {
                    Ok(line) => {
                        if sender.send(Self::stdout_to_event(line)).is_err() {
                            break;
                        }
                    }
                    Err(error) => {
                        let _ = sender.send((GameServerEvent::InError, error.to_string()));
                        break;
                    }
                }
            }
        });
    }

    fn spawn_stderr_reader(sender: Sender<GameServerMessage>, stderr: ChildStderr) {
        thread::spawn(move || {
            for line in BufReader::new(stderr).lines() {
                match line {
                    Ok(line) => {
                        if sender.send((GameServerEvent::StdError, line)).is_err() {
                            break;
                        }
                    }
                    Err(error) => {
                        let _ = sender.send((GameServerEvent::InError, error.to_string()));
                        break;
                    }
                }
            }
        });
    }

    pub fn is_running(&mut self) -> color_eyre::Result<bool> {
        Ok(self.child.try_wait()?.is_none())
    }

    pub fn stop(&mut self) -> color_eyre::Result<()> {
        if self.child.try_wait()?.is_none() {
            let _ = self.child.kill();
        }
        let _ = self.child.wait();
        Ok(())
    }
}

impl Drop for GameServerProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
