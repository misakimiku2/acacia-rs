pub mod db;
pub mod manager {
    pub mod player;
    pub mod player_attr;
    pub mod player_item;
    pub mod player_solution;
    pub mod player_str_attr;
}
pub mod models {
    mod player_attr;
    mod player_item;
    mod player_solution;
    mod player_str_attr;
    pub use player_attr::*;
    pub use player_item::*;
    pub use player_solution::*;
    pub use player_str_attr::*;
}

#[cfg(test)]
mod tests;
