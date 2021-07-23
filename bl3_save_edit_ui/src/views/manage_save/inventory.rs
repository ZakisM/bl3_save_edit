use iced::{
    button, pick_list, scrollable, text_input, tooltip, Align, Button, Color, Column, Container,
    Element, Length, PickList, Row, Scrollable, Text, TextInput, Tooltip,
};

use bl3_save_edit_core::bl3_save::bl3_serial::Bl3Serial;
use bl3_save_edit_core::parser::HeaderType;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::interaction::InteractionExt;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_margin::TextMargin;

#[derive(Debug, Default)]
pub struct InventoryState {
    pub items: Vec<InventoryItem>,
    pub scrollable_state: scrollable::State,
}

#[derive(Debug)]
pub enum InventoryMessage {}

#[derive(Debug, Clone)]
pub enum InventoryInteractionMessage {
    ItemPressed(usize),
}

#[derive(Debug)]
pub struct InventoryItem {
    id: usize,
    item: Bl3Serial,
    label: String,
    item_state: button::State,
}

impl InventoryItem {
    pub fn new(id: usize, item: Bl3Serial) -> Self {
        let balance_part = &item.balance_part;

        let label = format!(
            "{} (Level - {})",
            balance_part
                .name
                .clone()
                .unwrap_or_else(|| balance_part.ident.clone()),
            item.level
        );

        InventoryItem {
            id,
            label,
            item,
            item_state: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Button::new(
            &mut self.item_state,
            Text::new(&self.label).font(JETBRAINS_MONO).size(17),
        )
        .on_press(InteractionMessage::ManageSaveInteraction(
            ManageSaveInteractionMessage::Inventory(InventoryInteractionMessage::ItemPressed(
                self.id,
            )),
        ))
        .padding(10)
        .width(Length::Units(400))
        .style(Bl3UiStyle)
        .into_element()
    }
}

pub fn view(inventory_state: &mut InventoryState) -> Container<Message> {
    let inventory_items = inventory_state
        .items
        .iter_mut()
        .fold(Column::new().align_items(Align::Start), |curr, item| {
            curr.push(item.view())
        });

    let item_list = Container::new(
        Scrollable::new(&mut inventory_state.scrollable_state).push(inventory_items),
    )
    .width(Length::Units(400));

    let all_contents = Column::new().push(item_list).spacing(20);

    Container::new(all_contents).padding(30)
}
