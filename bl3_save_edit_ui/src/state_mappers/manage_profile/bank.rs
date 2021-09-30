use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use tracing::info;

use bl3_save_edit_core::bl3_profile::Bl3Profile;

use crate::views::item_editor::item_editor_list_item::ItemEditorListItem;
use crate::views::item_editor::{sort_items, ItemEditorStateExt};
use crate::views::manage_profile::ManageProfileState;

pub fn map_profile_to_bank_state(manage_profile_state: &mut ManageProfileState) -> Result<()> {
    let profile = &mut manage_profile_state.current_file;

    manage_profile_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .selected_item_index = 0;

    let bank_items = profile.profile_data.bank_items_mut();

    bank_items.par_sort_by(|a, b| sort_items(a, b));

    // This is important as we don't want to modify the original item list,
    // as we will use the original list to check which item's have been modified
    let bank_items_clone = bank_items.clone();

    *manage_profile_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .items_mut() = bank_items_clone
        .into_iter()
        .map(ItemEditorListItem::new)
        .collect();

    manage_profile_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .item_list_scrollable_state
        .snap_to(0.0);

    manage_profile_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .map_current_item_if_exists(|i| {
            i.editor.available_parts.scrollable_state.snap_to(0.0);
            i.editor.current_parts.scrollable_state.snap_to(0.0);
        })?;

    manage_profile_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .search_items_input
        .clear();

    Ok(())
}

pub fn map_bank_state_to_profile(
    manage_bank_state: &mut ManageProfileState,
    profile: &mut Bl3Profile,
) -> Result<()> {
    let bank_items = profile.profile_data.bank_items_mut();

    // Here we don't modify the save items just yet, we first modify
    // the mapped list and then set the save items equal to this mapped list
    for (i, new_item) in manage_bank_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .items()
        .iter()
        .enumerate()
    {
        if let Some(original_item) = bank_items.get(i) {
            // If the item we have edited has different serial number
            // Then we replace it
            if *original_item != new_item.item {
                info!("Replacing bank item at index: {}", i);

                bank_items.insert(i, new_item.item.to_owned());

                bank_items.remove(i + 1);
            } else {
                info!("Keeping existing bank item at index: {}", i);
            }
        } else {
            // Otherwise insert our new item in this slot
            info!("Inserting bank item at index: {}", i);

            bank_items.insert(i, new_item.item.to_owned());
        }
    }

    let new_bank_items = bank_items
        .par_iter()
        .map(|item| item.get_serial_number(true))
        .collect::<Result<Vec<_>>>()?;

    profile.profile_data.profile.bank_inventory_list = new_bank_items.into();

    Ok(())
}
