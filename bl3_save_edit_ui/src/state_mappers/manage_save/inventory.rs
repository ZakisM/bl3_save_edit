use crate::views::manage_save::inventory::InventoryItem;
use crate::views::manage_save::ManageSaveState;

pub fn map_inventory_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    let item_index = 0;

    manage_save_state.main_state.inventory_state.items = save
        .character_data
        .inventory_items
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, item)| {
            let is_active = i == item_index;

            InventoryItem::new(i, item, is_active)
        })
        .collect();

    map_save_to_inventory_state(manage_save_state, item_index);
}

pub fn map_save_to_inventory_state(manage_save_state: &mut ManageSaveState, item_index: usize) {
    let save = &manage_save_state.current_file;

    if let Some(item) = save.character_data.inventory_items.get(item_index) {
        manage_save_state.main_state.inventory_state.balance_input =
            item.balance_part.ident.clone();
    }
}
