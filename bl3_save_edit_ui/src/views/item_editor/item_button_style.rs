use iced::{button, Color};

pub struct ItemEditorButtonStyle {
    pub is_active: bool,
}

impl button::StyleSheet for ItemEditorButtonStyle {
    fn active(&self) -> button::Style {
        let (background, text_color) = if self.is_active {
            (
                Some(Color::from_rgb8(28, 28, 28).into()),
                Color::from_rgb8(255, 255, 255),
            )
        } else {
            (
                Some(Color::from_rgb8(22, 22, 22).into()),
                Color::from_rgb8(220, 220, 220),
            )
        };

        button::Style {
            background,
            text_color,
            border_width: 0.0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(30, 30, 30).into()),
            border_width: 0.0,
            text_color: Color::from_rgb8(255, 255, 255),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            border_width: 0.0,
            text_color: Color::from_rgb8(220, 220, 220),
            ..button::Style::default()
        }
    }
}

pub struct ItemEditorListButtonStyle;

impl button::StyleSheet for ItemEditorListButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(26, 26, 26).into()),
            text_color: Color::from_rgb8(224, 224, 224),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(46, 46, 46),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(29, 29, 29).into()),
            text_color: Color::from_rgb8(210, 210, 210),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(23, 23, 23).into()),
            text_color: Color::from_rgb8(210, 210, 210),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
            ..button::Style::default()
        }
    }
}

pub struct ItemEditorListNegativeButtonStyle;

impl button::StyleSheet for ItemEditorListNegativeButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(54, 29, 29).into()),
            text_color: Color::from_rgb8(240, 149, 149),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(61, 36, 36),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(54, 25, 25).into()),
            text_color: Color::from_rgb8(240, 125, 125),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(61, 41, 41),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(54, 29, 29).into()),
            text_color: Color::from_rgb8(240, 149, 149),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(61, 36, 36),
            ..button::Style::default()
        }
    }
}
