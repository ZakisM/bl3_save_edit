use std::fmt::Display;

use iced::{button, Button, Color, Element, HorizontalAlignment, Length, Text};
use strum::Display;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::resources::fonts::JETBRAINS_MONO_BOLD;
use crate::views::InteractionExt;

#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum AvailablePartType {
    #[strum(to_string = "Available Parts")]
    Parts,
    #[strum(to_string = "Available Anointments")]
    Anointments,
}

impl std::default::Default for AvailablePartType {
    fn default() -> Self {
        Self::Parts
    }
}

#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum CurrentPartType {
    #[strum(to_string = "Current Parts")]
    Parts,
    #[strum(to_string = "Current Anointments")]
    Anointments,
}

impl std::default::Default for CurrentPartType {
    fn default() -> Self {
        Self::Parts
    }
}

pub fn parts_tab_bar_button<'a, T>(
    state: &'a mut button::State,
    tab_bar_view: T,
    current_tab_bar_view: &T,
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
            .horizontal_alignment(HorizontalAlignment::Center),
    )
    // .on_press(InteractionMessage::ManageSaveInteraction(
    //     ManageSaveInteractionMessage::Inventory(on_press_message),
    // ))
    .on_press(on_press_message)
    .width(Length::Fill)
    .padding(15)
    .style(PartsTabBarActiveStyle);

    if tab_bar_view == *current_tab_bar_view {
        button.style(PartsTabBarActiveStyle).into_element()
    } else {
        button.style(PartsTabBarStyle).into_element()
    }
}

struct PartsTabBarActiveStyle;

impl button::StyleSheet for PartsTabBarActiveStyle {
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

struct PartsTabBarStyle;

impl button::StyleSheet for PartsTabBarStyle {
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
