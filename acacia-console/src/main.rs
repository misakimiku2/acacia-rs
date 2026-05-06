mod base {
    pub mod game_server {
        pub mod game_server_base;
        pub mod game_server_manager;
        pub mod game_server_process;
    }
}
mod ui {
    pub mod components {
        pub mod style;
    }
    pub mod pages;
}
mod terminal;

fn main() -> color_eyre::Result<()> {
    terminal::terminal_main()
}
