use anyhow::Result;
use tracing::info;

use bl3_save_edit_core::bl3_profile::Bl3Profile;

use crate::views::item_editor::item_editor_list_item::ItemEditorListItem;
use crate::views::item_editor::ItemEditorStateExt;
use crate::views::manage_profile::ManageProfileState;

pub fn map_profile_to_bank_state(manage_profile_state: &mut ManageProfileState) {
    let profile = &manage_profile_state.current_file;

    manage_profile_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .selected_item_index = 0;

    *manage_profile_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .items_mut() = profile
        .profile_data
        .bank_items()
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, item)| ItemEditorListItem::new(i, item))
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
        .map_current_item_if_exists(|i| i.editor.available_parts.scrollable_state.snap_to(0.0));
}

pub fn map_bank_state_to_profile(
    manage_bank_state: &mut ManageProfileState,
    profile: &mut Bl3Profile,
) -> Result<()> {
    for (i, edited_item) in manage_bank_state
        .profile_view_state
        .bank_state
        .item_editor_state
        .items()
        .iter()
        .enumerate()
    {
        if let Some(original_serial_number) =
            profile.profile_data.profile.bank_inventory_list.get(i)
        {
            // let original_serial_number = &original_item.item_serial_number;

            let edited_serial_number = edited_item.item.get_serial_number(true)?;

            // If the item we have edited has different serial number
            // Then we replace it
            if *original_serial_number != edited_serial_number {
                info!("Replacing bank item at index: {}", i);
                profile
                    .profile_data
                    .replace_bank_item(i, &edited_item.item)?;
            } else {
                info!("Keeping existing bank item at index: {}", i);
            }
        } else {
            // Otherwise insert our new item in this slot
            info!("Inserting bank item at index: {}", i);
            profile
                .profile_data
                .insert_bank_item(i, &edited_item.item)?;
        }
    }

    Ok(())
}
