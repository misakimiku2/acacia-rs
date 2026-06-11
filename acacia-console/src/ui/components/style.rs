use ratatui::style::{Color, Style};

pub const ACCENT_BAR: &str = "┃";

pub fn global_background(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(10, 10, 10), has_popup)
}

pub fn normal_text_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(238, 238, 238), has_popup)
}

pub fn normal_accent_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(0, 206, 209), has_popup)
}

pub fn normal_background_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(20, 20, 20), has_popup)
}

pub fn normal_background_active_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(30, 30, 30), has_popup)
}

pub fn muted_text_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(128, 128, 128), has_popup)
}

pub fn muted_accent_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(30, 30, 30), has_popup)
}

pub fn item_accent_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(72, 72, 72), has_popup)
}

pub fn item_selected_text_color(_has_popup: bool) -> Color {
    Color::Rgb(0, 0, 0)
}

pub fn item_selected_background_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(250, 178, 131), has_popup)
}

pub fn debug_accent_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(59, 120, 255), has_popup)
}

pub fn info_accent_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(19, 161, 14), has_popup)
}

pub fn warn_accent_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(193, 156, 0), has_popup)
}

pub fn error_accent_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(197, 15, 31), has_popup)
}

pub fn error_background_color(has_popup: bool) -> Color {
    popup_aware(Color::Rgb(103, 12, 20), has_popup)
}

pub fn selected_text_style(text_color: Color) -> Style {
    Style::default().fg(Color::Rgb(0, 0, 0)).bg(text_color)
}

pub fn input_cursor_style() -> Style {
    Style::default().fg(Color::Black).bg(Color::Rgb(238, 238, 238))
}

fn dim_color(color: Color, factor: f32) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb((r as f32 * factor).round() as u8, (g as f32 * factor).round() as u8, (b as f32 * factor).round() as u8),
        other => other,
    }
}

fn popup_aware(color: Color, has_popup: bool) -> Color {
    if has_popup { dim_color(color, 0.4) } else { color }
}
