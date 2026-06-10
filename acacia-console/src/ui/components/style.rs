use ratatui::style::{Color, Style};

pub const ACCENT_BAR: &str = "┃";
pub const OVERLAY_POPUP_BACKGROUND: Color = Color::Rgb(20, 20, 20);

pub fn global_background(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(4, 4, 4) } else { Color::Rgb(10, 10, 10) }
}

pub fn accent_color(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(0, 85, 86) } else { Color::Rgb(0, 206, 209) }
}

pub fn normal_text_color(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(98, 98, 98) } else { Color::Rgb(238, 238, 238) }
}

pub fn muted_text_color(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(53, 53, 53) } else { Color::Rgb(128, 128, 128) }
}

pub fn block_backgrounds(has_popup: bool) -> (Color, Color) {
    if has_popup {
        (Color::Rgb(8, 8, 8), Color::Rgb(12, 12, 12))
    } else {
        (Color::Rgb(20, 20, 20), Color::Rgb(30, 30, 30))
    }
}

pub fn selected_text_style(text_color: Color) -> Style {
    Style::default().fg(Color::Rgb(0, 0, 0)).bg(text_color)
}

pub fn input_cursor_style() -> Style {
    Style::default().fg(Color::Black).bg(Color::Rgb(238, 238, 238))
}

pub fn overlay_hint_color() -> Color {
    Color::Rgb(128, 128, 128)
}

pub fn selected_item_background(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(103, 73, 54) } else { Color::Rgb(250, 178, 131) }
}

pub fn selected_item_text_color(_has_popup: bool) -> Color {
    Color::Rgb(0, 0, 0)
}

pub fn command_popup_accent(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(29, 29, 29) } else { Color::Rgb(72, 72, 72) }
}

pub fn error_accent_color(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(79, 6, 12) } else { Color::Rgb(197, 15, 31) }
}

pub fn info_accent_color(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(8, 64, 6) } else { Color::Rgb(19, 161, 14) }
}

pub fn debug_accent_color(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(24, 48, 102) } else { Color::Rgb(59, 120, 255) }
}

pub fn warn_accent_color(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(77, 62, 0) } else { Color::Rgb(193, 156, 0) }
}

pub fn error_background_color(has_popup: bool) -> Color {
    if has_popup { Color::Rgb(41, 5, 8) } else { Color::Rgb(103, 12, 20) }
}
