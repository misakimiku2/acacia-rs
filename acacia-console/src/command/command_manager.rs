use crate::command::{
    command_base::{CommandItem, CommandItemAction},
    commands::{
        CommandKind,
        account::{account::AccountCommand, account_create::AccountCreateCommand, account_delete::AccountDeleteCommand},
        hello::HelloCommand,
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
        } else if AccountDeleteCommand::is_match(text) {
            if self.cached_kind != CommandKind::AccountDelete {
                self.cached_kind = CommandKind::AccountDelete;
                self.cached_items = AccountDeleteCommand::items();
            }
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

    pub fn execute(&mut self, text: &str) -> String {
        if HelloCommand::is_execute(text) {
            return HelloCommand::execute();
        }
        format!("Unknown command: {text}")
    }
}
