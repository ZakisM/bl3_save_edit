use iced::{button, Button, Color, Element, HorizontalAlignment, Length, Text};
use strum::Display;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::resources::fonts::JETBRAINS_MONO_BOLD;
use crate::views::manage_save::inventory::InventoryInteractionMessage;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;

#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum PartType {
    #[strum(to_string = "Available Parts")]
    AvailableParts,
    #[strum(to_string = "Available Anointment's")]
    AvailableAnointments,
}

impl std::default::Default for PartType {
    fn default() -> Self {
        Self::AvailableParts
    }
}

pub fn parts_tab_bar_button<'a>(
    state: &'a mut button::State,
    tab_bar_view: PartType,
    current_tab_bar_view: &PartType,
    on_press_message: InventoryInteractionMessage,
) -> Element<'a, Message> {
    let button = Button::new(
        state,
        Text::new(tab_bar_view.to_string())
            .font(JETBRAINS_MONO_BOLD)
            .size(17)
            .color(Color::from_rgb8(242, 203, 5))
            .width(Length::Fill)
            .horizontal_alignment(HorizontalAlignment::Center),
    )
    .on_press(InteractionMessage::ManageSaveInteraction(
        ManageSaveInteractionMessage::Inventory(on_press_message),
    ))
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
