use regex::Regex;
use std::sync::LazyLock;

use crate::base::api::AcaciaApi;

const EXECUTE_PATTERN: &str = r"^/hello";
static EXECUTE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(EXECUTE_PATTERN).unwrap());

#[derive(Clone)]
pub struct HelloCommand {}

impl HelloCommand {
    pub fn is_execute(text: &str) -> bool {
        EXECUTE_REGEX.is_match(text)
    }

    pub fn execute() -> String {
        match AcaciaApi::get::<String>("/") {
            Ok(body) => body,
            Err(error) => format!("[ERR] {error}"),
        }
    }
}
