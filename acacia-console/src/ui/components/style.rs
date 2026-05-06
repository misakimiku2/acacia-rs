use ratatui::style::Color;

pub fn global_background(overlay: bool) -> Color {
    if overlay { Color::Rgb(4, 4, 4) } else { Color::Rgb(10, 10, 10) }
}
