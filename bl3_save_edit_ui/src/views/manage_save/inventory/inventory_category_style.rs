use iced::{container, Color};

pub struct InventoryCategoryStyle;

impl container::StyleSheet for InventoryCategoryStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(23, 23, 23).into()),
            ..container::Style::default()
        }
    }
}
