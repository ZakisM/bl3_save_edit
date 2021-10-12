use anyhow::Result;

use bl3_save_edit_core::bl3_profile::Bl3Profile;

use crate::state_mappers::manage_profile;
use crate::views::manage_profile::ManageProfileState;

pub mod bank;
pub mod general;
pub mod keys;
pub mod profile;

pub fn map_all_states_to_profile(
    manage_profile_state: &mut ManageProfileState,
    current_file: &mut Bl3Profile,
) -> Result<bool> {
    manage_profile::general::map_general_state_to_profile(manage_profile_state, current_file);

    let guardian_data_injection_required =
        manage_profile::profile::map_profile_state_to_profile(manage_profile_state, current_file)?;

    manage_profile::keys::map_keys_state_to_profile(manage_profile_state, current_file)?;

    manage_profile::bank::map_bank_state_to_profile(manage_profile_state, current_file)?;

    Ok(guardian_data_injection_required)
}
