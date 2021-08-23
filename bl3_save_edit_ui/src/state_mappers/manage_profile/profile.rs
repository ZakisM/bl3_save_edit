use crate::views::manage_profile::ManageProfileState;

pub fn map_profile_to_profile_state(manage_profile_state: &mut ManageProfileState) {
    let profile = &manage_profile_state.current_file;

    manage_profile_state
        .profile_view_state
        .profile_state
        .guardian_rank_input = profile.profile_data.guardian_rank;

    manage_profile_state
        .profile_view_state
        .profile_state
        .guardian_rank_tokens_input = profile.profile_data.guardian_rank_tokens;
}
