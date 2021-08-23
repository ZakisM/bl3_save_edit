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

// pub fn map_general_state_to_profile(manage_profile_state: &mut ManageSaveState, save: &mut Bl3Save) {
//     save.character_data.character.save_game_guid =manage_profile_state
//         .save_view_state
//         .general_state
//         .guid_input
//         .clone();
//
//     save.character_data.character.save_game_id =
//         manage_profile_state.save_view_state.general_state.slot_input;
//
//     save.header_type =manage_profile_state
//         .save_view_state
//         .general_state
//         .save_type_selected;
// }
