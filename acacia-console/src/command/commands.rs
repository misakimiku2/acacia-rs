pub mod slash;
pub mod account {
    pub mod account;
    pub mod account_create;
}

#[derive(Clone, PartialEq)]
pub enum CommandKind {
    None,
    Slash,
    Account,
    AccountCreate,
}
