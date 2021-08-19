use std::mem;

use bl3_save_edit_core::file_helper::Bl3FileType;

use crate::bl3_ui::Bl3UiState;
use crate::bl3_ui::ViewState;
use crate::views::manage_save::main::MainTabBarView;
use crate::views::manage_save::ManageSaveView;

pub mod manage_save;

pub fn map_loaded_file_to_state(main_state: &mut Bl3UiState) {
    match &*main_state.loaded_files_selected {
        Bl3FileType::PcSave(save) | Bl3FileType::Ps4Save(save) => {
            //This file will be the one that gets modified when we press save.
            main_state.manage_save_state.current_file = save.clone();

            manage_save::general::map_save_to_general_state(&mut main_state.manage_save_state);

            manage_save::character::map_save_to_character_state(&mut main_state.manage_save_state);

            manage_save::inventory::map_save_to_inventory_state(&mut main_state.manage_save_state);

            manage_save::currency::map_currency_state(&mut main_state.manage_save_state);

            if mem::discriminant(&main_state.view_state)
                != mem::discriminant(&ViewState::ManageSave(ManageSaveView::TabBar(
                    MainTabBarView::General,
                )))
            {
                main_state.view_state =
                    ViewState::ManageSave(ManageSaveView::TabBar(MainTabBarView::General));
            }
        }
        Bl3FileType::PcProfile(p) | Bl3FileType::Ps4Profile(p) => (),
    }
}
