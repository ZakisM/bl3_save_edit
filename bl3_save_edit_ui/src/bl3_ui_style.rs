use iced::pick_list::Menu;
use iced::{button, checkbox, container, pick_list, searchable_pick_list, text_input, Color};

pub struct Bl3UiStyleNoBorder;

impl container::StyleSheet for Bl3UiStyleNoBorder {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(22, 22, 22).into(),
            ..container::Style::default()
        }
    }
}

pub struct Bl3UiStyleCustomNoBorder(pub Color);

impl container::StyleSheet for Bl3UiStyleCustomNoBorder {
    fn style(&self) -> container::Style {
        container::Style {
            background: self.0.into(),
            ..container::Style::default()
        }
    }
}

pub struct Bl3UiStyle;

impl container::StyleSheet for Bl3UiStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(22, 22, 22).into(),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(35, 35, 35),
            ..container::Style::default()
        }
    }
}

impl text_input::StyleSheet for Bl3UiStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Color::from_rgb8(23, 23, 23).into(),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(35, 35, 35),
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Color::from_rgb8(23, 23, 23).into(),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgba8(255, 255, 255, 0.1)
    }

    fn value_color(&self) -> Color {
        Color::from_rgb8(220, 220, 220)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgba8(255, 255, 255, 0.1)
    }
}

impl pick_list::StyleSheet for Bl3UiStyle {
    fn menu(&self) -> Menu {
        Menu {
            text_color: Color::from_rgb8(220, 220, 220),
            background: Color::from_rgb8(23, 23, 23).into(),
            border_width: 1.5,
            border_color: Color::from_rgb8(35, 35, 35),
            selected_background: Color::from_rgb8(35, 35, 35).into(),
            selected_text_color: Color::from_rgb8(220, 220, 220),
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: Color::from_rgb8(23, 23, 23).into(),
            text_color: Color::from_rgb8(220, 220, 220),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(35, 35, 35),
            icon_size: 0.5,
            placeholder_color: Default::default(),
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: Color::from_rgb8(30, 30, 30).into(),
            text_color: Color::from_rgb8(220, 220, 220),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
            icon_size: 0.5,
            placeholder_color: Default::default(),
        }
    }
}

impl searchable_pick_list::StyleSheet for Bl3UiStyle {
    fn menu(&self) -> Menu {
        Menu {
            text_color: Color::from_rgb8(220, 220, 220),
            background: Color::from_rgb8(23, 23, 23).into(),
            border_width: 1.5,
            border_color: Color::from_rgb8(35, 35, 35),
            selected_background: Color::from_rgb8(35, 35, 35).into(),
            selected_text_color: Color::from_rgb8(220, 220, 220),
        }
    }

    fn icon_size(&self) -> f32 {
        0.5
    }

    fn text_input_active(&self) -> text_input::Style {
        text_input::Style {
            background: Color::from_rgb8(23, 23, 23).into(),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(35, 35, 35),
        }
    }

    fn text_input_focused(&self) -> text_input::Style {
        text_input::Style {
            background: Color::from_rgb8(23, 23, 23).into(),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
        }
    }

    fn text_input_placeholder_color(&self) -> Color {
        Color::from_rgba8(255, 255, 255, 0.1)
    }

    fn text_input_value_color(&self) -> Color {
        Color::from_rgb8(220, 220, 220)
    }

    fn text_input_selection_color(&self) -> Color {
        Color::from_rgba8(255, 255, 255, 0.1)
    }

    fn pick_list_active(&self) -> pick_list::Style {
        pick_list::Style {
            background: Color::from_rgb8(23, 23, 23).into(),
            text_color: Color::from_rgb8(220, 220, 220),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(35, 35, 35),
            icon_size: 0.5,
            placeholder_color: Default::default(),
        }
    }

    fn pick_list_hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: Color::from_rgb8(30, 30, 30).into(),
            text_color: Color::from_rgb8(220, 220, 220),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
            icon_size: 0.5,
            placeholder_color: Default::default(),
        }
    }
}

impl checkbox::StyleSheet for Bl3UiStyle {
    fn active(&self, _: bool) -> checkbox::Style {
        checkbox::Style {
            background: Color::from_rgb8(35, 35, 35).into(),
            checkmark_color: Color::from_rgb8(220, 220, 220),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
        }
    }

    fn hovered(&self, _: bool) -> checkbox::Style {
        checkbox::Style {
            background: Color::from_rgb8(35, 35, 35).into(),
            checkmark_color: Color::from_rgb8(220, 220, 220),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
        }
    }
}

impl button::StyleSheet for Bl3UiStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(23, 23, 23).into()),
            text_color: Color::from_rgb8(220, 220, 220),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(35, 35, 35),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(27, 27, 27).into()),
            text_color: Color::from_rgb8(210, 210, 210),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(20, 20, 20).into()),
            text_color: Color::from_rgb8(210, 210, 210),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
            ..button::Style::default()
        }
    }
}

pub struct Bl3UiMenuBarStyle;

impl container::StyleSheet for Bl3UiMenuBarStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(35, 35, 35).into()),
            border_width: 1.5,
            border_color: Color::from_rgb8(25, 25, 25),
            ..container::Style::default()
        }
    }
}

pub struct Bl3UiContentStyle;

impl container::StyleSheet for Bl3UiContentStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            ..container::Style::default()
        }
    }
}

pub struct Bl3UiTooltipStyle;

impl container::StyleSheet for Bl3UiTooltipStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(220, 220, 220)),
            background: Color::from_rgb8(35, 35, 35).into(),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
        }
    }
}

pub struct Bl3UiPositiveButtonStyle;

impl button::StyleSheet for Bl3UiPositiveButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(29, 54, 35).into()),
            text_color: Color::from_rgb8(149, 240, 172),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(36, 61, 42),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(25, 54, 32).into()),
            text_color: Color::from_rgb8(125, 240, 153),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(41, 61, 46),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(29, 54, 35).into()),
            text_color: Color::from_rgb8(149, 240, 171),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(36, 61, 42),
            ..button::Style::default()
        }
    }
}
