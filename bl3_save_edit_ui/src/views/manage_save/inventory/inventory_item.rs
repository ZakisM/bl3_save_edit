use iced::{button, Button, Element, Length, Text};

use bl3_save_edit_core::bl3_save::bl3_item::Bl3Item;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::inventory::inventory_button_style::InventoryButtonStyle;
use crate::views::manage_save::inventory::InventoryInteractionMessage;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;

#[derive(Debug)]
pub struct InventoryItem {
    id: usize,
    pub item: Bl3Item,
    label: String,
    button_state: button::State,
}

impl InventoryItem {
    pub fn new(id: usize, item: Bl3Item) -> Self {
        let balance_part = &item.balance_part;

        let label = format!(
            "[Lvl {} {}] {}",
            item.level,
            item.item_type.to_string(),
            balance_part
                .name
                .clone()
                .unwrap_or_else(|| balance_part.ident.clone()),
        );

        InventoryItem {
            id,
            label,
            item,
            button_state: button::State::new(),
        }
    }

    pub fn view(&mut self, is_active: bool) -> Element<Message> {
        Button::new(
            &mut self.button_state,
            Text::new(&self.label).font(JETBRAINS_MONO).size(16),
        )
        .on_press(InteractionMessage::ManageSaveInteraction(
            ManageSaveInteractionMessage::Inventory(InventoryInteractionMessage::ItemPressed(
                self.id,
            )),
        ))
        .padding(10)
        .width(Length::Fill)
        .style(InventoryButtonStyle { is_active })
        .into_element()
    }
}
