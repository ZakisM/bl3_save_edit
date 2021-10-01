use std::fmt::Display;

use iced::alignment::Horizontal;
use iced::{button, Button, Color, Element, Length, Text};

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::resources::fonts::JETBRAINS_MONO_BOLD;
use crate::views::InteractionExt;

pub fn tab_bar_button<'a, T>(
    state: &'a mut button::State,
    tab_bar_view: T,
    current_tab_bar_type: &T,
    on_press_message: InteractionMessage,
    extra_title_content: Option<String>,
) -> Element<'a, Bl3Message>
where
    T: Display + PartialEq,
{
    let title = if let Some(extra_content) = extra_title_content {
        format!("{} {}", tab_bar_view, extra_content)
    } else {
        tab_bar_view.to_string()
    };

    let button = Button::new(
        state,
        Text::new(title)
            .font(JETBRAINS_MONO_BOLD)
            .size(17)
            .color(Color::from_rgb8(242, 203, 5))
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center),
    )
    .on_press(on_press_message)
    .width(Length::Fill)
    .padding(15)
    .style(TabBarButtonActiveStyle);

    if tab_bar_view == *current_tab_bar_type {
        button.style(TabBarButtonActiveStyle).into_element()
    } else {
        button.style(TabBarButtonStyle).into_element()
    }
}

struct TabBarButtonActiveStyle;

impl button::StyleSheet for TabBarButtonActiveStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(22, 22, 22).into()),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(22, 22, 22).into()),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(22, 22, 22).into()),
            ..button::Style::default()
        }
    }
}

struct TabBarButtonStyle;

impl button::StyleSheet for TabBarButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(26, 26, 26).into()),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(31, 31, 31).into()),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(28, 28, 28).into()),
            ..button::Style::default()
        }
    }
}
