use bl3_save_edit_core::bl3_profile::Bl3Profile;

use crate::views::manage_profile::general::ProfileGeneralInteractionMessage;
use crate::views::manage_profile::main::{
    ProfileTabBarInteractionMessage, ProfileTabBarView, ProfileViewState,
};
use crate::views::manage_profile::profile::ProfileProfileInteractionMessage;

pub mod general;
pub mod main;
pub mod profile;

#[derive(Debug, Default)]
pub struct ManageProfileState {
    pub profile_view_state: ProfileViewState,
    pub current_file: Bl3Profile,
}

#[derive(Debug, Clone)]
pub enum ManageProfileInteractionMessage {
    TabBar(ProfileTabBarInteractionMessage),
    General(ProfileGeneralInteractionMessage),
    Profile(ProfileProfileInteractionMessage),
    SaveFilePressed,
}

#[derive(Debug, PartialEq)]
pub enum ManageProfileView {
    TabBar(ProfileTabBarView),
}
