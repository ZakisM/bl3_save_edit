use iced::Container;

use crate::bl3_ui::{InteractionMessage, Message};
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

pub fn view(bank_state: &mut InventoryState) -> Container<Message> {
    item_editor::view(&mut bank_state.item_editor_state, |i| {
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::Inventory(
            SaveInventoryInteractionMessage::Editor(i),
        ))
    })
}
