use regex::Regex;
use serde_json::json;
use std::sync::LazyLock;

use crate::{
    base::api::AcaciaApi,
    command::command_base::{CommandItem, CommandItemAction},
};

static STATIC_ITEMS: LazyLock<Vec<CommandItem>> = LazyLock::new(|| {
    vec![
        CommandItem {
            name: Some("/account delete <uid> - delete a account".to_string()),
            desc: None,
            action: CommandItemAction::None,
        },
        CommandItem {
            name: Some("· uid".to_string()),
            desc: Some("required - the account uid".to_string()),
            action: CommandItemAction::None,
        },
    ]
});

const COMMAND_PATTERN: &str = r"^/account\s+delete\s+\S*$";
static COMMAND_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(COMMAND_PATTERN).unwrap());

const KEYWORD_PATTERN: &str = r"^/account\s+delete\s+(\S*)$";
static KEYWORD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(KEYWORD_PATTERN).unwrap());

const EXECUTE_PATTERN: &str = r"^/account\s+delete\s+(\S*)";
static EXECUTE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(EXECUTE_PATTERN).unwrap());

#[derive(Clone)]
pub struct AccountDeleteCommand {}

impl AccountDeleteCommand {
    pub fn is_match(text: &str) -> bool {
        COMMAND_REGEX.is_match(text)
    }

    pub fn keyword(text: &str) -> &str {
        if let Some(caps) = KEYWORD_REGEX.captures(text) {
            let keyword = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            return keyword;
        }
        ""
    }

    pub fn items() -> Vec<CommandItem> {
        let mut items = STATIC_ITEMS.to_vec();
        if let Ok(uids) = AcaciaApi::get::<Vec<u64>>("/account/list") {
            items.extend(uids.into_iter().map(|uid| CommandItem {
                name: Some(uid.to_string()),
                desc: Some("existing uid".to_string()),
                action: CommandItemAction::Insert(format!("/account delete {uid} ")),
            }));
        }
        items
    }

    pub fn is_execute(text: &str) -> bool {
        EXECUTE_REGEX.is_match(text)
    }

    pub fn execute(text: &str) -> String {
        let Some(caps) = EXECUTE_REGEX.captures(text) else {
            return format!("[ERR] Invalid parameter.");
        };
        let uid = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let Ok(uid) = uid.parse::<u64>() else {
            return format!("[ERR] uid must be u64, but got '{uid}'.");
        };
        match AcaciaApi::post::<serde_json::Value>("/account/delete", json!({"pid": uid}).to_string()) {
            Ok(_) => format!("Account deleted successfully."),
            Err(error) => format!("[ERR] {error}"),
        }
    }
}
