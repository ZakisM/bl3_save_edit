use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::manage_save::ManageSaveState;

pub fn map_general_state(save: &Bl3Save, manage_save_state: &mut ManageSaveState) {
    manage_save_state.main_state.general_state.guid_input =
        save.character_data.character.save_game_guid.clone();

    manage_save_state.main_state.general_state.slot_input =
        save.character_data.character.save_game_id;
}
