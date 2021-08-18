use crate::views::manage_save::ManageSaveState;

pub fn map_general_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    manage_save_state.main_state.general_state.filename_input = save.file_name.clone();

    manage_save_state.main_state.general_state.guid_input =
        save.character_data.character.save_game_guid.clone();

    manage_save_state.main_state.general_state.slot_input =
        save.character_data.character.save_game_id;

    manage_save_state
        .main_state
        .general_state
        .save_type_selected = save.header_type;
}
