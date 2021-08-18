use std::convert::TryInto;

use iced::{
    button, scrollable, text_input, Align, Button, Color, Column, Container, Length, Row,
    Scrollable, Text, TextInput,
};

use bl3_save_edit_core::bl3_save::bl3_item::{BalancePart, Bl3Item, InvDataPart, ManufacturerPart};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::inventory::available_parts::AvailablePartsIndex;
use crate::views::manage_save::inventory::current_parts::CurrentPartsIndex;
use crate::views::manage_save::inventory::inventory_item::InventoryListItem;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::text_margin::TextMargin;

pub mod available_parts;
pub mod current_parts;
pub mod delete_item_button_style;
pub mod inventory_button_style;
pub mod inventory_category_style;
pub mod inventory_item;
pub mod inventory_item_editor;

#[derive(Debug, Default)]
pub struct InventoryState {
    pub selected_item_index: usize,
    pub import_serial_input: String,
    pub import_serial_input_state: text_input::State,
    pub import_serial_button_state: button::State,
    items: Vec<InventoryListItem>,
    pub item_list_scrollable_state: scrollable::State,
}

impl InventoryState {
    pub fn items(&mut self) -> &Vec<InventoryListItem> {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<InventoryListItem> {
        &mut self.items
    }

    pub fn add_item(&mut self, item: Bl3Item) {
        self.items
            .push(InventoryListItem::new(self.items.len(), item));
    }

    pub fn remove_item(&mut self, remove_id: usize) {
        if remove_id <= self.items.len() {
            self.items.remove(remove_id);

            // Shift id of all existing items past remove_id to id - 1
            self.items.iter_mut().for_each(|i| {
                if i.id > remove_id {
                    i.id -= 1;
                }
            })
        }
    }
}

pub trait InventoryStateExt {
    fn map_current_item_if_exists<F>(&mut self, f: F)
    where
        F: FnOnce(&mut InventoryListItem);

    fn map_current_item_if_exists_to_editor_state(&mut self);
}

impl InventoryStateExt for InventoryState {
    fn map_current_item_if_exists<F>(&mut self, f: F)
    where
        F: FnOnce(&mut InventoryListItem),
    {
        if let Some(item) = self.items.get_mut(self.selected_item_index) {
            f(item);

            self.map_current_item_if_exists_to_editor_state();
        }
    }

    fn map_current_item_if_exists_to_editor_state(&mut self) {
        if let Some(curr_item) = self.items.get_mut(self.selected_item_index) {
            curr_item.editor.item_level_input = curr_item.item.level().try_into().unwrap_or(1);
            curr_item.editor.serial_input = curr_item
                .item
                .get_serial_number_base64(false)
                .unwrap_or_else(|_| "Unable to read serial, this item is invalid.".to_owned());
            curr_item.editor.balance_input_selected = curr_item.item.balance_part().clone();
            curr_item.editor.inv_data_input_selected = curr_item.item.inv_data_part().clone();
            curr_item.editor.manufacturer_input_selected =
                curr_item.item.manufacturer_part().clone();
        }
    }
}

#[derive(Debug)]
pub enum InventoryMessage {}

#[derive(Debug, Clone)]
pub enum InventoryInteractionMessage {
    ItemPressed(usize),
    AvailablePartPressed(AvailablePartsIndex),
    CurrentPartPressed(CurrentPartsIndex),
    SyncItemLevelWithCharacterLevel,
    ImportItemInputChanged(String),
    ImportItemFromSerial,
    ItemLevelInputChanged(i32),
    DeleteItem(usize),
    BalanceInputSelected(BalancePart),
    InvDataInputSelected(InvDataPart),
    ManufacturerInputSelected(ManufacturerPart),
}

pub fn view(inventory_state: &mut InventoryState) -> Container<Message> {
    let selected_item_index = inventory_state.selected_item_index;
    let number_of_items = inventory_state.items.len();

    let serial_importer = Row::new()
        .push(
            LabelledElement::create(
                "Import Serial",
                Length::Units(140),
                TextInput::new(
                    &mut inventory_state.import_serial_input_state,
                    "BL3(AwAAAABmboC7I9xAEzwShMJVX8nPYwsAAA==)",
                    &inventory_state.import_serial_input,
                    |s| {
                        InteractionMessage::ManageSaveInteraction(
                            ManageSaveInteractionMessage::Inventory(
                                InventoryInteractionMessage::ImportItemInputChanged(s),
                            ),
                        )
                    },
                )
                .font(JETBRAINS_MONO)
                .padding(10)
                .size(17)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .spacing(15)
            .width(Length::FillPortion(9))
            .align_items(Align::Center),
        )
        .push(
            Button::new(
                &mut inventory_state.import_serial_button_state,
                Text::new("Import").font(JETBRAINS_MONO_BOLD).size(17),
            )
            .on_press(InteractionMessage::ManageSaveInteraction(
                ManageSaveInteractionMessage::Inventory(
                    InventoryInteractionMessage::ImportItemFromSerial,
                ),
            ))
            .padding(10)
            .style(Bl3UiStyle)
            .into_element(),
        )
        .align_items(Align::Center);

    let general_options_row = Row::new().push(
        Container::new(serial_importer)
            .width(Length::Fill)
            .height(Length::Units(36))
            .style(Bl3UiStyle),
    );

    let mut item_editor = None;

    let inventory_items = inventory_state.items.iter_mut().enumerate().fold(
        Column::new().align_items(Align::Start),
        |inventory_items, (i, item)| {
            let is_active = i == selected_item_index;

            let (list_item_button, curr_item_editor) = item.view(is_active);

            if is_active {
                item_editor = curr_item_editor;
            }

            inventory_items.push(list_item_button)
        },
    );

    let item_editor = if let Some(item_editor) = item_editor {
        item_editor
    } else {
        Container::new(Text::new("Select something mate"))
    }
    .width(Length::FillPortion(7))
    .height(Length::Fill);

    let item_list = Container::new(
        Column::new()
            .push(
                Container::new(
                    TextMargin::new(format!("Items ({})", number_of_items), 2)
                        .0
                        .font(JETBRAINS_MONO_BOLD)
                        .size(17)
                        .color(Color::from_rgb8(242, 203, 5)),
                )
                .padding(10)
                .align_x(Align::Center)
                .width(Length::Fill)
                .style(Bl3UiStyle),
            )
            .push(
                Container::new(
                    Scrollable::new(&mut inventory_state.item_list_scrollable_state)
                        .push(inventory_items)
                        .height(Length::Fill),
                )
                .padding(1),
            ),
    )
    .width(Length::FillPortion(3))
    .height(Length::Fill)
    .style(Bl3UiStyle);

    let item_list_and_editor = Row::new().push(item_list).push(item_editor).spacing(20);

    let all_contents = Column::new()
        .push(general_options_row)
        .push(item_list_and_editor)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
