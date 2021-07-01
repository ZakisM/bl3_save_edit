use iced::pick_list::Menu;
use iced::{button, container, pick_list, Color};

pub const PRIMARY_COLOR: (u8, u8, u8) = (254, 226, 3);

pub struct Bl3UiStyle;

impl container::StyleSheet for Bl3UiStyle {
    fn style(&self) -> container::Style {
        container::Style {
            ..container::Style::default()
        }
    }
}
