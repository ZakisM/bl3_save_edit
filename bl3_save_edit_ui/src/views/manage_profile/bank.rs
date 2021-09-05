use iced::Container;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::views::item_editor;
use crate::views::item_editor::{ItemEditorInteractionMessage, ItemEditorState};
use crate::views::manage_profile::ManageProfileInteractionMessage;

#[derive(Debug, Default)]
pub struct BankState {
    pub item_editor_state: ItemEditorState,
}

#[derive(Debug, Clone)]
pub enum ProfileBankInteractionMessage {
    Editor(ItemEditorInteractionMessage),
}

pub fn view(bank_state: &mut BankState) -> Container<Bl3Message> {
    item_editor::view(&mut bank_state.item_editor_state, |i| {
        InteractionMessage::ManageProfileInteraction(ManageProfileInteractionMessage::Bank(
            ProfileBankInteractionMessage::Editor(i),
        ))
    })
}
