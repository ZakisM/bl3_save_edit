use iced::Container;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::views::item_editor;
use crate::views::item_editor::{ItemEditorInteractionMessage, ItemEditorState};
use crate::views::manage_save::ManageSaveInteractionMessage;

#[derive(Debug, Default)]
pub struct InventoryState {
    pub item_editor_state: ItemEditorState,
}

#[derive(Debug, Clone)]
pub enum SaveInventoryInteractionMessage {
    Editor(ItemEditorInteractionMessage),
}

pub fn view(inventory_state: &mut InventoryState) -> Container<Bl3Message> {
    item_editor::view(&mut inventory_state.item_editor_state, |i| {
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::Inventory(
            SaveInventoryInteractionMessage::Editor(i),
        ))
    })
}
