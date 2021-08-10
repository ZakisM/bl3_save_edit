use std::convert::TryInto;

use crate::views::manage_save::inventory::inventory_item::InventoryListItem;
use crate::views::manage_save::inventory::InventoryStateExt;
use crate::views::manage_save::ManageSaveState;

pub fn map_inventory_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    manage_save_state
        .main_state
        .inventory_state
        .selected_item_index = 0;

    manage_save_state.main_state.inventory_state.items = save
        .character_data
        .inventory_items
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, item)| InventoryListItem::new(i, item))
        .collect();

    manage_save_state
        .main_state
        .inventory_state
        .item_list_scrollable_state
        .snap_to(0.0);

    map_item_to_inventory_state(manage_save_state);
}

pub fn map_item_to_inventory_state(manage_save_state: &mut ManageSaveState) {
    //TODO: Snap to top for every scrollable in each state_mapper when it is required (including pick_list if possible)
    manage_save_state
        .main_state
        .inventory_state
        .map_current_item_if_exists(|i| i.editor.available_parts.scrollable_state.snap_to(0.0));

    let save = &manage_save_state.current_file;

    let selected_item_index = manage_save_state
        .main_state
        .inventory_state
        .selected_item_index;

    if let Some(item) = save.character_data.inventory_items.get(selected_item_index) {
        manage_save_state
            .main_state
            .inventory_state
            .selected_item_index = selected_item_index;

        if let Some(i) = manage_save_state
            .main_state
            .inventory_state
            .items
            .get_mut(selected_item_index)
        {
            if !i.has_mapped_from_save {
                i.editor.item_level_input = item.level.try_into().unwrap_or(1);
                i.editor.balance_input = item.balance_part.ident.clone();
                i.editor.inventory_data_input = item.inv_data.clone();
                i.editor.manufacturer_input = item.manufacturer.clone();

                if let Ok(base_64_serial) = item.get_serial_number_base64(false) {
                    i.editor.serial_input = base_64_serial;
                }

                i.has_mapped_from_save = true;
            }
        }
    }
}
