pub mod slash;
pub mod account {
    pub mod account;
    pub mod account_create;
    pub mod account_delete;
}

#[derive(Clone, PartialEq)]
pub enum CommandKind {
    None,
    Slash,
    Account,
    AccountCreate,
    AccountDelete,
}
