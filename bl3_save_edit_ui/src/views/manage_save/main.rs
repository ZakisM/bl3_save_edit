use iced::{
    button, container, svg, Align, Button, Color, Column, Container, HorizontalAlignment, Length,
    Row, Svg, Text,
};
use strum::Display;

use crate::bl3_ui::Message;
use crate::resources::fonts::{COMPACTA, JETBRAINS_MONO_BOLD};
use crate::resources::svgs::{CHARACTER, CURRENCY, FAST_TRAVEL, SETTINGS, VEHICLE};
use crate::views::manage_save::character::CharacterState;
use crate::views::manage_save::general::GeneralState;
use crate::views::manage_save::{character, general, ManageSaveMessage, ManageSaveState};

#[derive(Debug, Default)]
pub struct MainState {
    tab_bar_state: TabBarState,
    pub general_state: GeneralState,
    pub character_state: CharacterState,
}

#[derive(Debug, Default)]
pub struct TabBarState {
    general_button_state: button::State,
    character_button_state: button::State,
    vehicle_button_state: button::State,
    currency_button_state: button::State,
    fast_travel_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    TabBarGeneralPressed,
    TabBarCharacterPressed,
    TabBarVehiclePressed,
    TabBarCurrencyPressed,
    TabBarFastTravelPressed,
}

#[derive(Debug, Display, PartialEq)]
#[strum(serialize_all = "title_case")]
pub enum MainTabBarView {
    General,
    Character,
    Vehicle,
    Currency,
    FastTravel,
}

struct ManageSaveMenuBarStyle;

impl container::StyleSheet for ManageSaveMenuBarStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(35, 35, 35).into()),
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

struct ManageSaveTabBarStyle;

impl container::StyleSheet for ManageSaveTabBarStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(30, 30, 30).into()),
            border_width: 1.0,
            border_color: Color::from_rgb8(25, 25, 25),
            ..container::Style::default()
        }
    }
}

impl button::StyleSheet for ManageSaveTabBarStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(30, 30, 30).into()),
            text_color: Color::from_rgb8(210, 210, 210),
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

struct ManageSaveStyle;

impl container::StyleSheet for ManageSaveStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            ..container::Style::default()
        }
    }
}

pub fn view<'a>(
    manage_save_state: &'a mut ManageSaveState,
    tab_bar_view: &MainTabBarView,
) -> Container<'a, Message> {
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
        svg::Handle::from_memory(SETTINGS),
        100,
    );

    let character_button = tab_bar_button(
        &mut manage_save_state
            .main_state
            .tab_bar_state
            .character_button_state,
        MainTabBarView::Character,
        &tab_bar_view,
        MainMessage::TabBarCharacterPressed,
        svg::Handle::from_memory(CHARACTER),
        115,
    );

    let vehicle_button = tab_bar_button(
        &mut manage_save_state
            .main_state
            .tab_bar_state
            .vehicle_button_state,
        MainTabBarView::Vehicle,
        &tab_bar_view,
        MainMessage::TabBarVehiclePressed,
        svg::Handle::from_memory(VEHICLE),
        100,
    );

    let currency_button = tab_bar_button(
        &mut manage_save_state
            .main_state
            .tab_bar_state
            .currency_button_state,
        MainTabBarView::Currency,
        &tab_bar_view,
        MainMessage::TabBarCurrencyPressed,
        svg::Handle::from_memory(CURRENCY),
        105,
    );

    let fast_travel = tab_bar_button(
        &mut manage_save_state
            .main_state
            .tab_bar_state
            .fast_travel_button_state,
        MainTabBarView::FastTravel,
        &tab_bar_view,
        MainMessage::TabBarFastTravelPressed,
        svg::Handle::from_memory(FAST_TRAVEL),
        127,
    );

    let tab_bar = Container::new(
        Row::new()
            .push(general_button)
            .push(character_button)
            .push(vehicle_button)
            .push(currency_button)
            .push(fast_travel),
    )
    .width(Length::Fill)
    .style(ManageSaveTabBarStyle);

    let tab_content = match tab_bar_view {
        MainTabBarView::General => general::view(&mut manage_save_state.main_state.general_state),
        _ => character::view(&mut manage_save_state.main_state.character_state),
        // MainTabBarView::General => general::view(&mut manage_save_state.main_state.general_state),
        // MainTabBarView::Character => general::view(),
        // MainTabBarView::Vehicle => general::view(),
        // MainTabBarView::Currency => general::view(),
        // MainTabBarView::FastTravel => general::view(),
    };

    let all_contents = Column::new().push(menu_bar).push(tab_bar).push(tab_content);

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
    icon_handle: svg::Handle,
    length: u16,
) -> Button<'a, Message> {
    let icon = Svg::new(icon_handle)
        .height(Length::Units(17))
        .width(Length::Units(17));

    let button = Button::new(
        state,
        Row::new()
            .push(icon)
            .push(
                Text::new(tab_bar_view.to_string())
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .font(JETBRAINS_MONO_BOLD)
                    .size(18),
            )
            .padding(5)
            .spacing(10)
            .width(Length::Units(length))
            .align_items(Align::Center),
    )
    .on_press(Message::ManageSave(ManageSaveMessage::Main(
        on_press_message,
    )))
    .padding(5);

    if tab_bar_view == *current_tab_bar_view {
        button.style(ManageSaveTabBarActiveStyle)
    } else {
        button.style(ManageSaveTabBarStyle)
    }
}
