use crate::views::manage_profile::ManageProfileState;

pub fn map_profile_to_keys_state(manage_profile_state: &mut ManageProfileState) {
    let profile = &manage_profile_state.current_file;

    manage_profile_state
        .profile_view_state
        .keys_state
        .golden_keys_input = profile.profile_data.golden_keys();

    manage_profile_state
        .profile_view_state
        .keys_state
        .diamond_keys_input = profile.profile_data.diamond_keys();

    manage_profile_state
        .profile_view_state
        .keys_state
        .vault_card_1_keys_input = profile.profile_data.vault_card_1_keys();

    manage_profile_state
        .profile_view_state
        .keys_state
        .vault_card_1_chests_input = profile.profile_data.vault_card_1_chests();
}
