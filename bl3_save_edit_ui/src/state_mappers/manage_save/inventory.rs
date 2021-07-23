use crate::views::manage_save::inventory::InventoryItem;
use crate::views::manage_save::ManageSaveState;

pub fn map_inventory_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    manage_save_state.main_state.inventory_state.items = save
        .character_data
        .inventory_items
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, item)| InventoryItem::new(i, item))
        .collect();
}
