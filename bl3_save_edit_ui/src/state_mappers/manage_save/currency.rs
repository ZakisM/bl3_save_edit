use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::manage_save::ManageSaveState;

pub fn map_currency_state(save: &Bl3Save, manage_save_state: &mut ManageSaveState) {
    manage_save_state.main_state.currency_state.money_input = save.character_data.money;

    manage_save_state.main_state.currency_state.eridium_input = save.character_data.eridium;
}
