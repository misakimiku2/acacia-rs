use regex::Regex;
use std::sync::LazyLock;

use crate::command::command_base::{CommandItem, CommandItemAction};

static STATIC_ITEMS: LazyLock<Vec<CommandItem>> = LazyLock::new(|| {
    vec![
        CommandItem {
            name: Some("/hello".to_string()),
            desc: Some("request local hello endpoint".to_string()),
            action: CommandItemAction::Submit("/hello".to_string()),
        },
        CommandItem {
            name: Some("/account".to_string()),
            desc: Some("create or delete an account".to_string()),
            action: CommandItemAction::Insert("/account ".to_string()),
        },
    ]
});

const COMMAND_PATTERN: &str = r"^/\S*$";
static COMMAND_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(COMMAND_PATTERN).unwrap());

const KEYWORD_PATTERN: &str = r"^(\S*)$";
static KEYWORD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(KEYWORD_PATTERN).unwrap());

#[derive(Clone)]
pub struct SlashCommand {}

impl SlashCommand {
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
}
