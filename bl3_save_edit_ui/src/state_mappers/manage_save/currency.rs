use anyhow::Result;

use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::manage_save::ManageSaveState;

pub fn map_save_to_currency_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    manage_save_state.save_view_state.currency_state.money_input = save.character_data.money();

    manage_save_state
        .save_view_state
        .currency_state
        .eridium_input = save.character_data.eridium();
}

pub fn map_currrency_state_to_save(
    manage_save_state: &mut ManageSaveState,
    save: &mut Bl3Save,
) -> Result<()> {
    save.character_data
        .set_money(manage_save_state.save_view_state.currency_state.money_input)?;

    save.character_data.set_eridium(
        manage_save_state
            .save_view_state
            .currency_state
            .eridium_input,
    )?;

    Ok(())
}
