use anyhow::Result;

use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::state_mappers::manage_save;
use crate::views::manage_save::ManageSaveState;

pub mod character;
pub mod currency;
pub mod general;
pub mod inventory;
pub mod vehicle;

pub fn map_all_states_to_save(
    manage_save_state: &mut ManageSaveState,
    current_file: &mut Bl3Save,
) -> Result<()> {
    manage_save::general::map_general_state_to_save(manage_save_state, current_file);

    manage_save::character::map_character_state_to_save(manage_save_state, current_file)?;

    manage_save::inventory::map_inventory_state_to_save(manage_save_state, current_file)?;

    manage_save::currency::map_currrency_state_to_save(manage_save_state, current_file)?;

    manage_save::vehicle::map_vehicle_state_to_save(manage_save_state, current_file);

    Ok(())
}
