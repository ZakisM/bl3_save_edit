use iced::pick_list::Menu;
use iced::{button, checkbox, container, pick_list, text_input, Color};

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
            ..Menu::default()
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
            ..pick_list::Style::default()
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: Color::from_rgb8(35, 35, 35).into(),
            text_color: Color::from_rgb8(220, 220, 220),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
            icon_size: 0.5,
            ..pick_list::Style::default()
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
