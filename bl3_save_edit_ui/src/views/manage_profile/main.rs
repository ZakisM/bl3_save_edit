use iced::{button, svg, Column, Container, Length, Row};
use strum::Display;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::resources::svgs::{GENERAL, PROFILE};
use crate::views::manage_profile::general::GeneralState;
use crate::views::manage_profile::profile::ProfileState;
use crate::views::manage_profile::{
    general, profile, ManageProfileInteractionMessage, ManageProfileState,
};
use crate::views::{tab_bar_button, ManageTabBarStyle};

#[derive(Debug, Default)]
pub struct ProfileViewState {
    tab_bar_state: ProfileTabBarState,
    pub general_state: GeneralState,
    pub profile_state: ProfileState,
}

#[derive(Debug, Default)]
pub struct ProfileTabBarState {
    general_button_state: button::State,
    profile_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum ProfileTabBarInteractionMessage {
    General,
    Profile,
}

#[derive(Debug, Display, PartialEq)]
#[strum(serialize_all = "title_case")]
pub enum ProfileTabBarView {
    General,
    Profile,
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

    let tab_bar = Container::new(Row::new().push(general_button).push(profile_button))
        .width(Length::Fill)
        .style(ManageTabBarStyle);

    let tab_content = match tab_bar_view {
        ProfileTabBarView::General => {
            general::view(&mut manage_profile_state.profile_view_state.general_state)
        }
        ProfileTabBarView::Profile => {
            profile::view(&mut manage_profile_state.profile_view_state.profile_state)
        }
    };

    let all_contents = Column::new().push(tab_bar).push(tab_content);

    Container::new(all_contents)
        .width(Length::Fill)
        .height(Length::Fill)
}
