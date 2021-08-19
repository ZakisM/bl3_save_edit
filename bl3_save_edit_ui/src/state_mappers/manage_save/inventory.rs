use anyhow::Result;

use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::manage_save::inventory::inventory_item::InventoryListItem;
use crate::views::manage_save::inventory::InventoryStateExt;
use crate::views::manage_save::ManageSaveState;

pub fn map_save_to_inventory_state(manage_save_state: &mut ManageSaveState) {
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

pub fn map_inventory_state_to_save(
    manage_save_state: &mut ManageSaveState,
    save: &mut Bl3Save,
) -> Result<()> {
    // Remove all items first?
    save.character_data.character.inventory_items.clear();

    for (i, item) in manage_save_state
        .main_state
        .inventory_state
        .items()
        .iter()
        .enumerate()
    {
        save.character_data
            .add_inventory_item(i as i32, &item.item)?;
    }

    Ok(())
}
