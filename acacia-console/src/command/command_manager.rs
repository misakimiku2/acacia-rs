use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::command::{
    command_base::{CommandItem, CommandItemAction},
    commands::{
        CommandKind,
        account::{account::AccountCommand, account_create::AccountCreateCommand},
        slash::SlashCommand,
    },
};

pub struct CommandManager {
    pub cached_kind: CommandKind,
    pub cached_items: Vec<CommandItem>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            cached_kind: CommandKind::None,
            cached_items: Vec::new(),
        }
    }

    pub fn command(&mut self, text: &str) -> Vec<CommandItem> {
        let mut keyword = "";
        let mut result = Vec::new();

        if SlashCommand::is_match(text) {
            if self.cached_kind != CommandKind::Slash {
                self.cached_kind = CommandKind::Slash;
                self.cached_items = SlashCommand::items();
            }
            keyword = SlashCommand::keyword(text);
        } else if AccountCommand::is_match(text) {
            if self.cached_kind != CommandKind::Account {
                self.cached_kind = CommandKind::Account;
                self.cached_items = AccountCommand::items();
            }
            keyword = AccountCommand::keyword(text);
        } else if AccountCreateCommand::is_match(text) {
            if self.cached_kind != CommandKind::AccountCreate {
                self.cached_kind = CommandKind::AccountCreate;
                self.cached_items = AccountCreateCommand::items();
            }
            keyword = AccountCreateCommand::keyword(text);
        } else {
            self.cached_kind = CommandKind::None;
            self.cached_items = Vec::new();
        }

        for item in &self.cached_items {
            if item.action == CommandItemAction::None {
                result.push(item.clone());
                continue;
            }
            if let Some(name) = &item.name {
                if name.starts_with(keyword) {
                    result.push(item.clone());
                }
            } else {
                result.push(item.clone());
            }
        }
        result
    }

    pub fn run(&mut self, text: &str) -> String {
        if text.starts_with("/hello") {
            let result = match get_http_text("127.0.0.1:52000", "/") {
                Ok(text) => text,
                Err(error) => format!("Request failed: {error}"),
            };
            return result;
        }
        format!("Unknown command: {text}")
    }
}

fn get_http_text(address: &str, path: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(address)?;
    stream.write_all(format!("GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n").as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response.split_once("\r\n\r\n").map_or(response.clone(), |(_, body)| body.to_owned()))
}
