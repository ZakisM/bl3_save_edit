use iced::{button, svg, Column, Container, Length, Row};
use strum::Display;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::resources::svgs::{GENERAL, KEYS, PROFILE};
use crate::views::manage_profile::general::GeneralState;
use crate::views::manage_profile::keys::KeysState;
use crate::views::manage_profile::profile::ProfileState;
use crate::views::manage_profile::{
    general, keys, profile, ManageProfileInteractionMessage, ManageProfileState,
};
use crate::views::{tab_bar_button, ManageTabBarStyle};

#[derive(Debug, Default)]
pub struct ProfileViewState {
    tab_bar_state: ProfileTabBarState,
    pub general_state: GeneralState,
    pub profile_state: ProfileState,
    pub keys_state: KeysState,
}

#[derive(Debug, Default)]
pub struct ProfileTabBarState {
    general_button_state: button::State,
    profile_button_state: button::State,
    keys_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum ProfileTabBarInteractionMessage {
    General,
    Profile,
    Keys,
}

#[derive(Debug, Display, PartialEq)]
#[strum(serialize_all = "title_case")]
pub enum ProfileTabBarView {
    General,
    Profile,
    Keys,
}

pub fn view<'a>(
    manage_profile_state: &'a mut ManageProfileState,
    tab_bar_view: &ProfileTabBarView,
) -> Container<'a, Message> {
    let general_button = tab_bar_button(
        &mut manage_profile_state
            .profile_view_state
            .tab_bar_state
            .general_button_state,
        ProfileTabBarView::General,
        tab_bar_view,
        InteractionMessage::ManageProfileInteraction(ManageProfileInteractionMessage::TabBar(
            ProfileTabBarInteractionMessage::General,
        )),
        svg::Handle::from_memory(GENERAL),
        100,
    );

    let profile_button = tab_bar_button(
        &mut manage_profile_state
            .profile_view_state
            .tab_bar_state
            .profile_button_state,
        ProfileTabBarView::Profile,
        tab_bar_view,
        InteractionMessage::ManageProfileInteraction(ManageProfileInteractionMessage::TabBar(
            ProfileTabBarInteractionMessage::Profile,
        )),
        svg::Handle::from_memory(PROFILE),
        100,
    );

    let keys_button = tab_bar_button(
        &mut manage_profile_state
            .profile_view_state
            .tab_bar_state
            .keys_button_state,
        ProfileTabBarView::Keys,
        tab_bar_view,
        InteractionMessage::ManageProfileInteraction(ManageProfileInteractionMessage::TabBar(
            ProfileTabBarInteractionMessage::Keys,
        )),
        svg::Handle::from_memory(KEYS),
        75,
    );

    let tab_bar = Container::new(
        Row::new()
            .push(general_button)
            .push(profile_button)
            .push(keys_button),
    )
    .width(Length::Fill)
    .style(ManageTabBarStyle);

    let tab_content = match tab_bar_view {
        ProfileTabBarView::General => {
            general::view(&mut manage_profile_state.profile_view_state.general_state)
        }
        ProfileTabBarView::Profile => {
            profile::view(&mut manage_profile_state.profile_view_state.profile_state)
        }
        ProfileTabBarView::Keys => {
            keys::view(&mut manage_profile_state.profile_view_state.keys_state)
        }
    };

    let all_contents = Column::new().push(tab_bar).push(tab_content);

    Container::new(all_contents)
        .width(Length::Fill)
        .height(Length::Fill)
}
