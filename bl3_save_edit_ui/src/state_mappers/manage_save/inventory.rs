use anyhow::Result;
use either::Either;
use tracing::info;

use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::item_editor::item_editor_list_item::ItemEditorListItem;
use crate::views::item_editor::ItemEditorStateExt;
use crate::views::manage_save::ManageSaveState;

pub fn map_save_to_inventory_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .selected_item_index = 0;

    let save_inventory_items = if manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .item_list_is_reverse_order
    {
        Either::Left(save.character_data.inventory_items().iter().rev())
    } else {
        Either::Right(save.character_data.inventory_items().iter())
    };

    *manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .items_mut() = save_inventory_items
        .cloned()
        .map(ItemEditorListItem::new)
        .collect();

    manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .item_list_scrollable_state
        .snap_to(0.0);

    manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .map_current_item_if_exists(|i| {
            i.editor.available_parts.scrollable_state.snap_to(0.0);
            i.editor.current_parts.scrollable_state.snap_to(0.0);
        });

    manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .search_items_input
        .clear();
}

pub fn map_inventory_state_to_save(
    manage_save_state: &mut ManageSaveState,
    save: &mut Bl3Save,
) -> Result<()> {
    // Don't reverse items in our save
    if manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .item_list_is_reverse_order
    {
        manage_save_state
            .save_view_state
            .inventory_state
            .item_editor_state
            .items_mut()
            .reverse();
    }

    for (i, edited_item) in manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .items()
        .iter()
        .enumerate()
    {
        if let Some(original_item) = save.character_data.character.inventory_items.get(i) {
            let original_serial_number = &original_item.item_serial_number;

            let edited_serial_number = edited_item.item.get_serial_number(true)?;

            // If the item we have edited has different serial number
            // Then we replace it
            if *original_serial_number != edited_serial_number {
                info!("Replacing item at index: {}", i);
                save.character_data
                    .replace_inventory_item(i as i32, i, &edited_item.item)?;
            } else {
                info!("Keeping existing item at index: {}", i);
            }
        } else {
            // Otherwise insert our new item in this slot
            info!("Inserting item at index: {}", i);
            save.character_data
                .insert_inventory_item(i as i32, i, &edited_item.item)?;
        }
    }

    Ok(())
}
