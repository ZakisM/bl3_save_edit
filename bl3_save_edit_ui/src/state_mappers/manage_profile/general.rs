use bl3_save_edit_core::bl3_profile::Bl3Profile;

use crate::views::manage_profile::ManageProfileState;

pub fn map_profile_to_general_state(manage_profile_state: &mut ManageProfileState) {
    let profile = &manage_profile_state.current_file;

    manage_profile_state
        .profile_view_state
        .general_state
        .filename_input = profile.file_name.clone();

    manage_profile_state
        .profile_view_state
        .general_state
        .profile_type_selected = profile.header_type;
}

pub fn map_general_state_to_profile(
    manage_profile_state: &mut ManageProfileState,
    profile: &mut Bl3Profile,
) {
    profile.file_name = manage_profile_state
        .profile_view_state
        .general_state
        .filename_input
        .clone();

    profile.header_type = manage_profile_state
        .profile_view_state
        .general_state
        .profile_type_selected;
}
