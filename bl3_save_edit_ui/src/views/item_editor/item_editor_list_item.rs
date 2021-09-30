use std::convert::TryInto;

use anyhow::{bail, Result};
use iced::{button, Button, Container, Element, Length};

use bl3_save_edit_core::bl3_item::Bl3Item;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::views::item_editor::editor::Editor;
use crate::views::item_editor::item_button_style::ItemEditorButtonStyle;
use crate::views::item_editor::{list_item_contents, ItemEditorInteractionMessage};
use crate::views::InteractionExt;

#[derive(Debug, Default)]
pub struct ItemEditorListItem {
    pub item: Bl3Item,
    button_state: button::State,
    pub editor: Editor,
}

impl ItemEditorListItem {
    pub fn new(item: Bl3Item) -> Self {
        ItemEditorListItem {
            item,
            ..Default::default()
        }
    }

    pub fn map_item_to_editor(&mut self) -> Result<()> {
        if let Ok(serial) = self.item.get_serial_number_base64(false) {
            self.editor.item_level_input = self.item.level().try_into().unwrap_or(1);
            self.editor.serial_input = serial;
            self.editor.balance_input_selected = self.item.balance_part().clone();
            self.editor.inv_data_input_selected = self.item.inv_data_part().clone();
            self.editor.manufacturer_input_selected = self.item.manufacturer_part().clone();
        } else {
            self.editor = Editor::default();
            bail!("failed to create a valid serial - this means that somehow this item is invalid.")
        }

        Ok(())
    }

    pub fn view<F>(
        &mut self,
        id: usize,
        is_active: bool,
        interaction_message: F,
    ) -> (Element<Bl3Message>, Option<Container<Bl3Message>>)
    where
        F: Fn(ItemEditorInteractionMessage) -> InteractionMessage + 'static + Copy,
    {
        let item_content = list_item_contents::view(&self.item);

        let item_editor = if is_active {
            Some(self.editor.view(id, &self.item, interaction_message))
        } else {
            None
        };

        (
            Button::new(&mut self.button_state, Container::new(item_content))
                .on_press(interaction_message(
                    ItemEditorInteractionMessage::ItemPressed(id),
                ))
                .padding(9)
                .width(Length::Fill)
                .style(ItemEditorButtonStyle { is_active })
                .into_element(),
            item_editor,
        )
    }
}
