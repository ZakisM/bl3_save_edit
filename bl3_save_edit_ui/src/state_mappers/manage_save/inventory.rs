use anyhow::Result;
use rayon::slice::ParallelSliceMut;
use tracing::info;

use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::item_editor::item_editor_list_item::ItemEditorListItem;
use crate::views::item_editor::{sort_items, ItemEditorStateExt};
use crate::views::manage_save::ManageSaveState;

pub fn map_save_to_inventory_state(manage_save_state: &mut ManageSaveState) -> Result<()> {
    let save = &mut manage_save_state.current_file;

    manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .selected_item_index = 0;

    let mut inventory_items = save
        .character_data
        .inventory_items()
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, item)| ItemEditorListItem::new(i, item))
        .collect::<Vec<_>>();

    inventory_items.par_sort_by(|a, b| {
        let a_item = &a.item;
        let b_item = &b.item;

        sort_items(a_item, b_item)
    });

    *manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .items_mut() = inventory_items;

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
        })?;

    manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .search_items_input
        .clear();

    Ok(())
}

pub fn map_inventory_state_to_save(
    manage_save_state: &mut ManageSaveState,
    save: &mut Bl3Save,
) -> Result<()> {
    let mut inventory_items = manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .items()
        .iter()
        .map(|i| (i.index, &i.item))
        .collect::<Vec<_>>();

    inventory_items.par_sort_by_key(|(i, _)| *i);

    for (i, edited_item) in inventory_items {
        if let Some(original_item) = save.character_data.character.inventory_items.get(i) {
            let original_serial_number = &original_item.item_serial_number;

            let edited_serial_number = edited_item.get_serial_number(true)?;

            // If the item we have edited has different serial number
            // Then we replace it
            if *original_serial_number != edited_serial_number {
                info!("Replacing item at index: {}", i);

                save.character_data
                    .replace_inventory_item(i as i32, i, edited_item)?;
            } else {
                info!("Keeping existing item at index: {}", i);
            }
        } else {
            // Otherwise insert our new item in this slot
            info!("Inserting item at index: {}", i);

            save.character_data
                .insert_inventory_item(i as i32, i, edited_item)?;
        }
    }

    Ok(())
}
