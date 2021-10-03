use anyhow::Result;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use tracing::info;

use bl3_save_edit_core::bl3_save::character_data::CharacterData;
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

    let equipped_items = save
        .character_data
        .character
        .equipped_inventory_list
        .iter()
        .flat_map(|e| {
            let index = e.inventory_list_index as usize;
            let item = save
                .character_data
                .inventory_items()
                .get(index)
                .map(|i| i.to_owned());

            item.map(|item| (e.slot_data_path.to_owned(), item))
        })
        .collect::<Vec<_>>();

    let inventory_items = save.character_data.inventory_items_mut();

    inventory_items.par_sort_by(|a, b| sort_items(a, b));

    // This is important as we don't want to modify the original item list,
    // as we will use the original list to check which item's have been modified
    let inventory_items_clone = save.character_data.inventory_items().clone();

    let mut active_item_set = false;

    for (equipped_slot_path, item) in equipped_items {
        //Find the new index of the sorted item
        if let Some(pos) = inventory_items_clone.iter().position(|i| i == &item) {
            // Set our active weapon also
            if !active_item_set && equipped_slot_path.contains("Weapon") {
                save.character_data.character.active_weapon_list = vec![pos as i32];

                active_item_set = true;
            }

            if let Some(existing) = save
                .character_data
                .character
                .equipped_inventory_list
                .iter_mut()
                .find(|e| e.slot_data_path == equipped_slot_path)
            {
                existing.inventory_list_index = pos as i32;
            }
        }
    }

    *manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .items_mut() = inventory_items_clone
        .into_iter()
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
    let inventory_items = save.character_data.inventory_items_mut();

    // Here we don't modify the save items just yet, we first modify
    // the mapped list and then set the save items equal to this mapped list
    for (i, new_item) in manage_save_state
        .save_view_state
        .inventory_state
        .item_editor_state
        .items()
        .iter()
        .enumerate()
    {
        if let Some(original_item) = inventory_items.get(i) {
            // If the item we have edited has different serial number
            // Then we replace it
            if *original_item != new_item.item {
                info!("Replacing item at index: {}", i);

                inventory_items.insert(i, new_item.item.to_owned());

                inventory_items.remove(i + 1);
            } else {
                info!("Keeping existing item at index: {}", i);
            }
        } else {
            // Otherwise insert our new item in this slot
            info!("Inserting item at index: {}", i);

            inventory_items.insert(i, new_item.item.to_owned());
        }
    }

    let new_inventory_items = inventory_items
        .par_iter()
        .enumerate()
        .map(|(i, item)| CharacterData::create_inventory_item(i as i32, item, true))
        .collect::<Result<Vec<_>>>()?;

    save.character_data.character.inventory_items = new_inventory_items.into();

    Ok(())
}
