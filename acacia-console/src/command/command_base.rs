#[derive(Clone)]
pub struct CommandItem {
    pub name: Option<String>,
    pub desc: Option<String>,
    pub action: CommandItemAction,
}

#[derive(Clone, PartialEq)]
pub enum CommandItemAction {
    None,
    Insert(String),
    Submit(String),
}
