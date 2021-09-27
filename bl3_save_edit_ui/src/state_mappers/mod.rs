use std::mem;

use anyhow::Result;

use bl3_save_edit_core::file_helper::Bl3FileType;

use crate::bl3_ui::Bl3Application;
use crate::bl3_ui::ViewState;
use crate::views::manage_profile::main::ProfileTabBarView;
use crate::views::manage_profile::ManageProfileView;
use crate::views::manage_save::main::SaveTabBarView;
use crate::views::manage_save::ManageSaveView;

pub mod manage_profile;
pub mod manage_save;

pub fn map_loaded_file_to_state(main_state: &mut Bl3Application) -> Result<()> {
    match &*main_state.loaded_files_selected {
        Bl3FileType::PcSave(save) | Bl3FileType::Ps4Save(save) => {
            //This file will be the one that gets modified when we press save.
            main_state.manage_save_state.current_file = save.clone();

            manage_save::general::map_save_to_general_state(&mut main_state.manage_save_state);

            manage_save::character::map_save_to_character_state(&mut main_state.manage_save_state);

            manage_save::inventory::map_save_to_inventory_state(&mut main_state.manage_save_state)?;

            manage_save::currency::map_save_to_currency_state(&mut main_state.manage_save_state);

            manage_save::vehicle::map_save_to_vehicle_state(&mut main_state.manage_save_state);

            if mem::discriminant(&main_state.view_state)
                != mem::discriminant(&ViewState::ManageSave(ManageSaveView::TabBar(
                    SaveTabBarView::General,
                )))
            {
                main_state.view_state =
                    ViewState::ManageSave(ManageSaveView::TabBar(SaveTabBarView::General));
            }
        }
        Bl3FileType::PcProfile(profile) | Bl3FileType::Ps4Profile(profile) => {
            main_state.manage_profile_state.current_file = profile.clone();

            manage_profile::general::map_profile_to_general_state(
                &mut main_state.manage_profile_state,
            );

            manage_profile::profile::map_profile_to_profile_state(
                &mut main_state.manage_profile_state,
            );

            manage_profile::keys::map_profile_to_keys_state(&mut main_state.manage_profile_state);

            manage_profile::bank::map_profile_to_bank_state(&mut main_state.manage_profile_state)?;

            if mem::discriminant(&main_state.view_state)
                != mem::discriminant(&ViewState::ManageProfile(ManageProfileView::TabBar(
                    ProfileTabBarView::General,
                )))
            {
                main_state.view_state =
                    ViewState::ManageProfile(ManageProfileView::TabBar(ProfileTabBarView::General));
            }
        }
    }

    Ok(())
}
