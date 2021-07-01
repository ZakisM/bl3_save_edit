use iced::image::viewer::Renderer;
use iced::{
    button, container, Align, Button, Color, Column, Container, HorizontalAlignment, Length, Row,
    Text, VerticalAlignment,
};
use strum::Display;

use crate::bl3_ui::Message;
use crate::bl3_ui::Message::ManageSave;
use crate::fonts::{CABIN, CABIN_BOLD, COMPACTA};
use crate::views::manage_save::{ManageSaveMessage, ManageSaveState};

#[derive(Debug, Default)]
pub struct MainState {
    tab_bar_state: TabBarState,
}

#[derive(Debug, Default)]
pub struct TabBarState {
    general_button_state: button::State,
    character_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    TabBarGeneralPressed,
    TabBarCharacterPressed,
}

#[derive(Debug, Display, PartialEq)]
pub enum MainTabBarView {
    General,
    Character,
}

struct ManageSaveMenuBarStyle;

impl container::StyleSheet for ManageSaveMenuBarStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(30, 30, 30).into()),
            border_width: 1.5,
            border_color: Color::from_rgb8(25, 25, 25),
            ..container::Style::default()
        }
    }
}

struct ManageSaveTabBarActiveStyle;

impl button::StyleSheet for ManageSaveTabBarActiveStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(20, 20, 20).into()),
            text_color: Color::from_rgb8(255, 199, 38),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(20, 20, 20).into()),
            text_color: Color::from_rgb8(255, 199, 38),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(20, 20, 20).into()),
            text_color: Color::from_rgb8(255, 199, 38),
            ..button::Style::default()
        }
    }
}

struct ManageSaveTabBarStyle;

impl container::StyleSheet for ManageSaveTabBarStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            border_width: 1.0,
            border_color: Color::from_rgb8(20, 20, 20),
            ..container::Style::default()
        }
    }
}

impl button::StyleSheet for ManageSaveTabBarStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            text_color: Color::from_rgb8(210, 210, 210),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(51, 51, 43).into()),
            text_color: Color::from_rgb8(255, 199, 38),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(41, 41, 36).into()),
            text_color: Color::from_rgb8(204, 160, 27),
            ..button::Style::default()
        }
    }
}

struct ManageSaveStyle;

impl container::StyleSheet for ManageSaveStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(20, 20, 20).into()),
            ..container::Style::default()
        }
    }
}

pub fn view(
    manage_save_state: &mut ManageSaveState,
    tab_bar_view: MainTabBarView,
) -> Container<Message> {
    let title = Text::new("Borderlands 3 Save Edit".to_uppercase())
        .font(COMPACTA)
        .size(48)
        .color(Color::from_rgb8(242, 203, 5))
        .width(Length::Fill)
        .horizontal_alignment(HorizontalAlignment::Left);

    let menu_bar = Container::new(
        Row::new()
            .push(title)
            .spacing(25)
            .align_items(Align::Center),
    )
    .padding(20)
    .width(Length::Fill)
    .style(ManageSaveMenuBarStyle);

    let general_button = tab_bar_button(
        &mut manage_save_state
            .main_state
            .tab_bar_state
            .general_button_state,
        MainTabBarView::General,
        &tab_bar_view,
        MainMessage::TabBarGeneralPressed,
    );

    let character_button = tab_bar_button(
        &mut manage_save_state
            .main_state
            .tab_bar_state
            .character_button_state,
        MainTabBarView::Character,
        &tab_bar_view,
        MainMessage::TabBarCharacterPressed,
    );

    let tab_bar = Container::new(Row::new().push(general_button).push(character_button))
        .width(Length::Fill)
        .style(ManageSaveTabBarStyle);

    let all_contents = Column::new()
        .align_items(Align::Center)
        .push(menu_bar)
        .push(tab_bar);

    Container::new(all_contents)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(ManageSaveStyle)
}

fn tab_bar_button<'a>(
    state: &'a mut button::State,
    tab_bar_view: MainTabBarView,
    current_tab_bar_view: &MainTabBarView,
    on_press_message: MainMessage,
) -> Button<'a, Message> {
    let mut button = Button::new(
        state,
        Text::new(tab_bar_view.to_string().to_uppercase())
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(CABIN_BOLD)
            .size(18),
    )
    .on_press(Message::ManageSave(ManageSaveMessage::Main(
        on_press_message,
    )))
    .width(Length::Units(125))
    .padding(10);

    if tab_bar_view == *current_tab_bar_view {
        button.style(ManageSaveTabBarActiveStyle)
    } else {
        button.style(ManageSaveTabBarStyle)
    }
}
