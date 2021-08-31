use iced::{button, Color};

pub struct DeleteItemButtonStyle;

impl button::StyleSheet for DeleteItemButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(54, 29, 29).into()),
            text_color: Color::from_rgb8(240, 149, 149),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(61, 36, 36),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(54, 25, 25).into()),
            text_color: Color::from_rgb8(240, 125, 125),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(61, 41, 41),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(54, 29, 29).into()),
            text_color: Color::from_rgb8(240, 149, 149),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(61, 36, 36),
            ..button::Style::default()
        }
    }
}
