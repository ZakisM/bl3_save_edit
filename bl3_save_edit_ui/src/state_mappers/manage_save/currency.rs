use crate::views::manage_save::ManageSaveState;

pub fn map_currency_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    manage_save_state.main_state.currency_state.money_input = save.character_data.money;

    manage_save_state.main_state.currency_state.eridium_input = save.character_data.eridium;
}
