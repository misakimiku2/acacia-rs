mod base {
    pub mod game_server {
        pub mod game_server_base;
        pub mod game_server_manager;
        pub mod game_server_process;
    }
    pub mod api;
    pub mod time_tick;
    pub mod utils;
}
pub mod command {
    pub mod command_base;
    pub mod command_manager;
    pub mod commands;
}
mod ui {
    pub mod components {
        pub mod input;
        pub mod popup;
        pub mod selection;
        pub mod style;
        pub mod text;
    }
    pub mod pages;
}
mod terminal;

fn main() -> color_eyre::Result<()> {
    terminal::terminal_main()
}
