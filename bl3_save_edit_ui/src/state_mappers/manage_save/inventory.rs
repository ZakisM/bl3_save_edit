use crate::views::manage_save::inventory::inventory_item::InventoryListItem;
use crate::views::manage_save::inventory::InventoryStateExt;
use crate::views::manage_save::ManageSaveState;

pub fn map_inventory_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    manage_save_state
        .main_state
        .inventory_state
        .selected_item_index = 0;

    *manage_save_state.main_state.inventory_state.items_mut() = save
        .character_data
        .inventory_items()
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

    manage_save_state
        .main_state
        .inventory_state
        .map_current_item_if_exists(|i| i.editor.available_parts.scrollable_state.snap_to(0.0));
}
