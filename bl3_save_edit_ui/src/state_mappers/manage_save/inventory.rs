use crate::views::manage_save::inventory::InventoryItem;
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
        .map(|(i, item)| InventoryItem::new(i, item))
        .collect();

    manage_save_state
        .main_state
        .inventory_state
        .item_list_scrollable_state
        .snap_to(0.0);

    map_item_to_manage_save_state(manage_save_state);
}

pub fn map_item_to_manage_save_state(manage_save_state: &mut ManageSaveState) {
    //TODO: Snap to top for every scrollable in each state_mapper when it is required (including pick_list if possible)
    manage_save_state
        .main_state
        .inventory_state
        .available_parts
        .scrollable_state
        .snap_to(0.0);

    manage_save_state
        .main_state
        .inventory_state
        .current_parts
        .scrollable_state
        .snap_to(0.0);

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

        manage_save_state.main_state.inventory_state.balance_input =
            item.balance_part.ident.clone();

        manage_save_state
            .main_state
            .inventory_state
            .inventory_data_input = item.inv_data.clone();

        manage_save_state
            .main_state
            .inventory_state
            .manufacturer_input = item.manufacturer.clone();
    }
}
