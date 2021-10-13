use anyhow::Result;
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

    let mut bank_items = profile
        .profile_data
        .bank_items()
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, item)| ItemEditorListItem::new(i, item))
        .collect::<Vec<_>>();

    bank_items.par_sort_by(|a, b| {
        let a_item = &a.item;
        let b_item = &b.item;

        sort_items(a_item, b_item)
    });

    *manage_profile_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .items_mut() = bank_items;

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
    let mut bank_items = manage_bank_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .items()
        .iter()
        .map(|i| (i.index, &i.item))
        .collect::<Vec<_>>();

    bank_items.par_sort_by_key(|(i, _)| *i);

    // Here we don't modify the save items just yet, we first modify
    // the mapped list and then set the save items equal to this mapped list
    for (i, edited_item) in bank_items {
        if let Some(original_serial_number) =
            profile.profile_data.profile.bank_inventory_list.get(i)
        {
            let edited_serial_number = edited_item.get_serial_number(true)?;

            // If the item we have edited has different serial number
            // Then we replace it
            if *original_serial_number != edited_serial_number {
                info!("Replacing bank item at index: {}", i);

                profile.profile_data.replace_bank_item(i, edited_item)?;
            } else {
                info!("Keeping existing bank item at index: {}", i);
            }
        } else {
            // Otherwise insert our new item in this slot
            info!("Inserting bank item at index: {}", i);

            profile.profile_data.insert_bank_item(i, edited_item)?;
        }
    }

    Ok(())
}
