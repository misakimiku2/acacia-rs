use regex::Regex;
use std::sync::LazyLock;

use crate::command::command_base::{CommandItem, CommandItemAction};

static STATIC_ITEMS: LazyLock<Vec<CommandItem>> = LazyLock::new(|| {
    vec![
        CommandItem {
            name: Some("/account create <uid> [token] - create a account".to_string()),
            desc: None,
            action: CommandItemAction::None,
        },
        CommandItem {
            name: Some("· uid".to_string()),
            desc: Some("required - the account uid".to_string()),
            action: CommandItemAction::None,
        },
        CommandItem {
            name: Some("· token".to_string()),
            desc: Some("optional - the account token".to_string()),
            action: CommandItemAction::None,
        },
    ]
});

const COMMAND_PATTERN: &str = r"^/account\s+create\s+";
static COMMAND_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(COMMAND_PATTERN).unwrap());

const KEYWORD_PATTERN: &str = r"^/account\s+create\s+(\S*)";
static KEYWORD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(KEYWORD_PATTERN).unwrap());

const EXECUTE_PATTERN: &str = r"^/account\s+create\s+(\S*)(?:\s+(\S+))?$";
static EXECUTE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(EXECUTE_PATTERN).unwrap());

#[derive(Clone)]
pub struct AccountCreateCommand {}

impl AccountCreateCommand {
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
        STATIC_ITEMS.to_vec()
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

        let _token = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        crate::acacia_api_post!("/account/create", {"pid": uid})
    }
}
