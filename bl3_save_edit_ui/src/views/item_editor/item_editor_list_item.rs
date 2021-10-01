use std::convert::TryInto;

use anyhow::{bail, Result};
use iced::alignment::Horizontal;
use iced::{button, Button, Container, Element, Length, Row, Text};

use bl3_save_edit_core::bl3_item::Bl3Item;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::resources::fonts::JETBRAINS_MONO_BOLD;
use crate::views::item_editor::editor::Editor;
use crate::views::item_editor::item_button_style::{
    ItemEditorButtonStyle, ItemEditorListButtonStyle, ItemEditorListNegativeButtonStyle,
};
use crate::views::item_editor::{list_item_contents, ItemEditorInteractionMessage};
use crate::views::InteractionExt;

#[derive(Debug, Default)]
pub struct ItemEditorListItem {
    pub item: Bl3Item,
    list_button_state: button::State,
    duplicate_button_state: button::State,
    share_button_state: button::State,
    delete_button_state: button::State,
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
        let action_row = Row::new()
            .push(
                Button::new(
                    &mut self.duplicate_button_state,
                    Text::new("Duplicate")
                        .font(JETBRAINS_MONO_BOLD)
                        .size(17)
                        .horizontal_alignment(Horizontal::Center),
                )
                .on_press(interaction_message(
                    ItemEditorInteractionMessage::DuplicateItem(id),
                ))
                .padding(5)
                .width(Length::Units(100))
                .style(ItemEditorListButtonStyle),
            )
            .push(
                Button::new(
                    &mut self.share_button_state,
                    Text::new("Share")
                        .font(JETBRAINS_MONO_BOLD)
                        .size(17)
                        .horizontal_alignment(Horizontal::Center),
                )
                .on_press(interaction_message(
                    ItemEditorInteractionMessage::ShareItem(id),
                ))
                .padding(5)
                .width(Length::Units(100))
                .style(ItemEditorListButtonStyle),
            )
            .push(
                Button::new(
                    &mut self.delete_button_state,
                    Text::new("Delete")
                        .font(JETBRAINS_MONO_BOLD)
                        .size(17)
                        .horizontal_alignment(Horizontal::Center),
                )
                .on_press(interaction_message(
                    ItemEditorInteractionMessage::DeleteItem(id),
                ))
                .padding(5)
                .width(Length::Units(100))
                .style(ItemEditorListNegativeButtonStyle),
            )
            .width(Length::Fill)
            .spacing(10);

        let item_content = list_item_contents::view(&self.item).push(action_row);

        let item_editor = if is_active {
            Some(self.editor.view(&self.item, interaction_message))
        } else {
            None
        };

        (
            Button::new(&mut self.list_button_state, Container::new(item_content))
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
