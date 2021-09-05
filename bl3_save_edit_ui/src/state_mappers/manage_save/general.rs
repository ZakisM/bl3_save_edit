use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::manage_save::ManageSaveState;

pub fn map_save_to_general_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    manage_save_state
        .save_view_state
        .general_state
        .filename_input = save.file_name.clone();

    manage_save_state.save_view_state.general_state.guid_input =
        save.character_data.character.save_game_guid.clone();

    manage_save_state.save_view_state.general_state.slot_input =
        save.character_data.character.save_game_id;

    manage_save_state
        .save_view_state
        .general_state
        .save_type_selected = save.header_type;
}

pub fn map_general_state_to_save(manage_save_state: &mut ManageSaveState, save: &mut Bl3Save) {
    save.file_name = manage_save_state
        .save_view_state
        .general_state
        .filename_input
        .clone();

    save.character_data.character.save_game_guid = manage_save_state
        .save_view_state
        .general_state
        .guid_input
        .clone();

    save.character_data.character.save_game_id =
        manage_save_state.save_view_state.general_state.slot_input;

    save.header_type = manage_save_state
        .save_view_state
        .general_state
        .save_type_selected;
}
