use std::fmt::Display;

use iced::alignment::Horizontal;
use iced::{button, container, svg, Alignment, Button, Color, Element, Length, Row, Svg, Text};

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::resources::fonts::JETBRAINS_MONO_BOLD;

pub mod choose_save_directory;
pub mod initialization;
pub mod item_editor;
pub mod loading;
pub mod manage_profile;
pub mod manage_save;
pub mod settings;
pub mod tab_bar_button;

pub const NO_SEARCH_RESULTS_FOUND_MESSAGE: &str = "No results found.";

pub trait InteractionExt<'a, T>
where
    T: Into<Element<'a, InteractionMessage>>,
{
    fn into_element(self) -> Element<'a, Bl3Message>;
}

impl<'a, T> InteractionExt<'a, T> for T
where
    T: Into<Element<'a, InteractionMessage>>,
{
    fn into_element(self) -> Element<'a, Bl3Message> {
        let element: Element<'a, InteractionMessage> = self.into();
        element.map(Bl3Message::Interaction)
    }
}

fn tab_bar_button<'a, V: Display + PartialEq>(
    state: &'a mut button::State,
    tab_bar_view: V,
    current_tab_bar_view: &V,
    on_press_message: InteractionMessage,
    icon_handle: svg::Handle,
    length: u16,
) -> Element<'a, Bl3Message> {
    let icon = Svg::new(icon_handle)
        .height(Length::Units(17))
        .width(Length::Units(17));

    let button = Button::new(
        state,
        Row::new()
            .push(icon)
            .push(
                Text::new(tab_bar_view.to_string())
                    .horizontal_alignment(Horizontal::Center)
                    .font(JETBRAINS_MONO_BOLD)
                    .size(18),
            )
            .padding(5)
            .spacing(10)
            .width(Length::Units(length))
            .align_items(Alignment::Center),
    )
    .on_press(on_press_message)
    .padding(5);

    if tab_bar_view == *current_tab_bar_view {
        button.style(ManageTabBarActiveStyle).into_element()
    } else {
        button.style(ManageTabBarStyle).into_element()
    }
}

struct ManageTabBarActiveStyle;

impl button::StyleSheet for ManageTabBarActiveStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            text_color: Color::from_rgb8(242, 203, 5),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            text_color: Color::from_rgb8(255, 199, 38),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            text_color: Color::from_rgb8(255, 199, 38),
            ..button::Style::default()
        }
    }
}

struct ManageTabBarStyle;

impl container::StyleSheet for ManageTabBarStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(30, 30, 30).into()),
            border_width: 1.0,
            border_color: Color::from_rgb8(25, 25, 25),
            ..container::Style::default()
        }
    }
}

impl button::StyleSheet for ManageTabBarStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(30, 30, 30).into()),
            text_color: Color::from_rgb8(220, 220, 220),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(35, 35, 35).into()),
            text_color: Color::from_rgb8(210, 210, 210),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(32, 32, 32).into()),
            text_color: Color::from_rgb8(210, 210, 210),
            ..button::Style::default()
        }
    }
}
