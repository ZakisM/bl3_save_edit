use anyhow::Result;

use bl3_save_edit_core::bl3_profile::profile_currency::ProfileCurrency;
use bl3_save_edit_core::bl3_profile::Bl3Profile;

use crate::views::manage_profile::ManageProfileState;

pub fn map_profile_to_keys_state(manage_profile_state: &mut ManageProfileState) {
    let profile = &manage_profile_state.current_file;

    manage_profile_state
        .profile_view_state
        .keys_state
        .golden_keys_input = profile.profile_data.golden_keys();

    manage_profile_state
        .profile_view_state
        .keys_state
        .diamond_keys_input = profile.profile_data.diamond_keys();

    manage_profile_state
        .profile_view_state
        .keys_state
        .vault_card_1_keys_input = profile.profile_data.vault_card_1_keys();

    manage_profile_state
        .profile_view_state
        .keys_state
        .vault_card_1_chests_input = profile.profile_data.vault_card_1_chests();

    manage_profile_state
        .profile_view_state
        .keys_state
        .vault_card_2_keys_input = profile.profile_data.vault_card_2_keys();

    manage_profile_state
        .profile_view_state
        .keys_state
        .vault_card_2_chests_input = profile.profile_data.vault_card_2_chests();

    manage_profile_state
        .profile_view_state
        .keys_state
        .vault_card_3_keys_input = profile.profile_data.vault_card_3_keys();

    manage_profile_state
        .profile_view_state
        .keys_state
        .vault_card_3_chests_input = profile.profile_data.vault_card_3_chests();
}

pub fn map_keys_state_to_profile(
    manage_profile_state: &mut ManageProfileState,
    profile: &mut Bl3Profile,
) -> Result<()> {
    let keys_state = &manage_profile_state.profile_view_state.keys_state;

    profile
        .profile_data
        .set_currency(&ProfileCurrency::GoldenKey, keys_state.golden_keys_input)?;

    profile
        .profile_data
        .set_currency(&ProfileCurrency::DiamondKey, keys_state.diamond_keys_input)?;

    profile.profile_data.set_currency(
        &ProfileCurrency::VaultCardOneId,
        keys_state.vault_card_1_keys_input,
    )?;

    profile
        .profile_data
        .set_vault_card_chests(1, keys_state.vault_card_1_chests_input);

    profile.profile_data.set_currency(
        &ProfileCurrency::VaultCardTwoId,
        keys_state.vault_card_2_keys_input,
    )?;

    profile
        .profile_data
        .set_vault_card_chests(2, keys_state.vault_card_2_chests_input);

    profile.profile_data.set_currency(
        &ProfileCurrency::VaultCardThreeId,
        keys_state.vault_card_3_keys_input,
    )?;

    profile
        .profile_data
        .set_vault_card_chests(3, keys_state.vault_card_3_chests_input);

    Ok(())
}
