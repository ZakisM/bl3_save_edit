use std::convert::TryInto;
use std::time::Duration;

use anyhow::Result;
use iced::{
    button, scrollable, text_input, tooltip, Align, Button, Color, Column, Command, Container,
    Length, Row, Scrollable, Text, Tooltip,
};

use bl3_save_edit_core::bl3_item::{
    BalancePart, Bl3Item, InvDataPart, ManufacturerPart, MAX_BL3_ITEM_ANOINTMENTS,
    MAX_BL3_ITEM_PARTS,
};
use bl3_save_edit_core::bl3_profile::Bl3Profile;
use bl3_save_edit_core::bl3_save::character_data::MAX_CHARACTER_LEVEL;
use bl3_save_edit_core::bl3_save::Bl3Save;
use bl3_save_edit_core::resources::INVENTORY_SERIAL_DB;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::item_editor::available_parts::AvailablePartTypeIndex;
use crate::views::item_editor::current_parts::CurrentPartTypeIndex;
use crate::views::item_editor::item_editor_list_item::ItemEditorListItem;
use crate::views::item_editor::parts_tab_bar::{AvailablePartType, CurrentPartType};
use crate::views::{InteractionExt, NO_SEARCH_RESULTS_FOUND_MESSAGE};
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::notification::{Notification, NotificationSentiment};
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_input_limited::TextInputLimited;
use crate::widgets::text_margin::TextMargin;

pub mod available_parts;
pub mod current_parts;
pub mod editor;
pub mod extra_part_info;
pub mod item_button_style;
pub mod item_editor_list_item;
pub mod parts_tab_bar;

#[derive(Debug, Default)]
pub struct ItemEditorState {
    pub selected_item_index: usize,
    pub create_item_button_state: button::State,
    pub import_serial_input: String,
    pub import_serial_input_state: text_input::State,
    pub all_item_levels_input: i32,
    pub all_item_levels_input_state: text_input::State,
    pub all_item_levels_button_state: button::State,
    pub import_serial_button_state: button::State,
    items: Vec<ItemEditorListItem>,
    pub item_list_scrollable_state: scrollable::State,
}

impl ItemEditorState {
    pub fn items(&mut self) -> &Vec<ItemEditorListItem> {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<ItemEditorListItem> {
        &mut self.items
    }

    pub fn add_item(&mut self, item: Bl3Item) {
        self.items
            .push(ItemEditorListItem::new(self.items.len(), item));
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

pub trait ItemEditorStateExt {
    fn map_current_item_if_exists<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ItemEditorListItem);

    fn map_current_item_if_exists_result<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut ItemEditorListItem) -> Result<()>;

    fn map_current_item_if_exists_to_editor_state(&mut self);
}

impl ItemEditorStateExt for ItemEditorState {
    fn map_current_item_if_exists<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ItemEditorListItem),
    {
        if let Some(item) = self.items.get_mut(self.selected_item_index) {
            f(item);

            self.map_current_item_if_exists_to_editor_state();
        }
    }

    fn map_current_item_if_exists_result<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut ItemEditorListItem) -> Result<()>,
    {
        if let Some(item) = self.items.get_mut(self.selected_item_index) {
            f(item)?;

            self.map_current_item_if_exists_to_editor_state();
        }

        Ok(())
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
pub enum ItemEditorFileType<'a> {
    Save(&'a mut Bl3Save),
    ProfileBank(&'a mut Bl3Profile),
    // ProfileLostLoot(Bl3Profile),
}

#[derive(Debug, Clone)]
pub enum ItemEditorInteractionMessage {
    ItemPressed(usize),
    ShowAllAvailablePartsSelected(bool),
    AvailablePartsSearchInputChanged(String),
    AvailablePartsTabPressed,
    AvailableAnointmentsTabPressed,
    CurrentPartsTabPressed,
    CurrentAnointmentsTabPressed,
    AvailablePartPressed(AvailablePartTypeIndex),
    AvailableAnointmentPressed(AvailablePartTypeIndex),
    CurrentPartPressed(CurrentPartTypeIndex),
    CurrentAnointmentPressed(CurrentPartTypeIndex),
    ImportItem(String),
    CreateItemPressed,
    ImportItemFromSerialPressed,
    AllItemLevel(i32),
    AllItemLevelDebounced,
    ItemLevel(i32),
    DeleteItem(usize),
    DuplicateItem(usize),
    BalanceInputSelected(BalancePart),
    BalanceSearchInputChanged(String),
    InvDataInputSelected(InvDataPart),
    InvDataSearchInputChanged(String),
    ManufacturerSearchInputChanged(String),
    ManufacturerInputSelected(ManufacturerPart),
}

#[derive(Debug)]
pub struct ItemEditorInteractionResponse {
    pub notification: Option<Notification>,
    pub command: Option<Command<ItemEditorInteractionMessage>>,
}

impl ItemEditorInteractionMessage {
    pub fn update_state(
        self,
        item_editor_state: &mut ItemEditorState,
        item_editor_file_type: ItemEditorFileType,
    ) -> ItemEditorInteractionResponse {
        let mut notification = None;
        let mut command = None;

        match self {
            ItemEditorInteractionMessage::ItemPressed(item_index) => {
                item_editor_state.selected_item_index = item_index;

                item_editor_state.map_current_item_if_exists(|i| {
                    i.editor.available_parts.part_type_index =
                        available_parts::AvailablePartTypeIndex {
                            category_index: 0,
                            part_index: 0,
                        }
                });
            }
            ItemEditorInteractionMessage::ShowAllAvailablePartsSelected(selected) => {
                item_editor_state.map_current_item_if_exists(|i| {
                    i.editor.available_parts.show_all_available_parts = selected;
                })
            }
            ItemEditorInteractionMessage::AvailablePartsSearchInputChanged(search_input) => {
                item_editor_state.map_current_item_if_exists(|i| {
                    i.editor.available_parts.search_input = search_input.to_lowercase();
                });
            }
            ItemEditorInteractionMessage::AvailablePartsTabPressed => item_editor_state
                .map_current_item_if_exists(|i| {
                    i.editor.available_parts.scrollable_state.snap_to(0.0);
                    i.editor.available_parts.search_input = "".to_owned();
                    i.editor.available_parts.parts_tab_view = AvailablePartType::Parts;
                }),
            ItemEditorInteractionMessage::AvailableAnointmentsTabPressed => item_editor_state
                .map_current_item_if_exists(|i| {
                    i.editor.available_parts.scrollable_state.snap_to(0.0);
                    i.editor.available_parts.search_input = "".to_owned();
                    i.editor.available_parts.parts_tab_view = AvailablePartType::Anointments;
                }),
            ItemEditorInteractionMessage::AvailablePartPressed(available_part_type_index) => {
                let selected_item_index = item_editor_state.selected_item_index;

                if let Some(current_item) =
                    item_editor_state.items_mut().get_mut(selected_item_index)
                {
                    if let Some(item_parts) = &mut current_item.item.item_parts {
                        if item_parts.parts().len() < MAX_BL3_ITEM_PARTS {
                            let part_selected = current_item
                                .editor
                                .available_parts
                                .parts
                                .get(available_part_type_index.category_index)
                                .and_then(|p| p.parts.get(available_part_type_index.part_index));

                            if let Some(part_selected) = part_selected {
                                let part_inv_key = &item_parts.part_inv_key;

                                if let Ok(bl3_part) = INVENTORY_SERIAL_DB
                                    .get_part_by_short_name(part_inv_key, &part_selected.part.name)
                                {
                                    if let Err(e) = current_item.item.add_part(bl3_part) {
                                        let msg = format!("Failed to add part to item: {}", e);

                                        notification = Some(Notification::new(
                                            msg,
                                            NotificationSentiment::Negative,
                                        ));
                                    }

                                    item_editor_state.map_current_item_if_exists(|i| {
                                        i.editor.available_parts.part_type_index =
                                            available_part_type_index
                                    });
                                }
                            }
                        }
                    }
                }
            }
            ItemEditorInteractionMessage::AvailableAnointmentPressed(available_part_type_index) => {
                let selected_item_index = item_editor_state.selected_item_index;

                if let Some(current_item) =
                    item_editor_state.items_mut().get_mut(selected_item_index)
                {
                    if let Some(item_parts) = &mut current_item.item.item_parts {
                        if item_parts.generic_parts().len() < MAX_BL3_ITEM_ANOINTMENTS {
                            let anointment_selected = current_item
                                .editor
                                .available_parts
                                .parts
                                .get(available_part_type_index.category_index)
                                .and_then(|p| p.parts.get(available_part_type_index.part_index));

                            if let Some(anointment_selected) = anointment_selected {
                                if let Ok(bl3_part) = INVENTORY_SERIAL_DB.get_part_by_short_name(
                                    "InventoryGenericPartData",
                                    &anointment_selected.part.name,
                                ) {
                                    if let Err(e) = current_item.item.add_generic_part(bl3_part) {
                                        let msg = format!("Failed to add part to item: {}", e);

                                        notification = Some(Notification::new(
                                            msg,
                                            NotificationSentiment::Negative,
                                        ));
                                    }

                                    item_editor_state.map_current_item_if_exists(|i| {
                                        i.editor.available_parts.part_type_index =
                                            available_part_type_index
                                    });
                                }
                            }
                        }
                    }
                }
            }
            ItemEditorInteractionMessage::CurrentPartsTabPressed => item_editor_state
                .map_current_item_if_exists(|i| {
                    i.editor.current_parts.scrollable_state.snap_to(0.0);
                    i.editor.current_parts.parts_tab_view = CurrentPartType::Parts
                }),
            ItemEditorInteractionMessage::CurrentAnointmentsTabPressed => item_editor_state
                .map_current_item_if_exists(|i| {
                    i.editor.current_parts.scrollable_state.snap_to(0.0);
                    i.editor.current_parts.parts_tab_view = CurrentPartType::Anointments
                }),
            ItemEditorInteractionMessage::CurrentPartPressed(current_part_type_index) => {
                let selected_item_index = item_editor_state.selected_item_index;

                if let Some(current_item) =
                    item_editor_state.items_mut().get_mut(selected_item_index)
                {
                    let part_selected = current_item
                        .editor
                        .current_parts
                        .parts
                        .get(current_part_type_index.category_index)
                        .and_then(|p| p.parts.get(current_part_type_index.part_index));

                    if let Some(part_selected) = part_selected {
                        if let Err(e) = current_item.item.remove_part(&part_selected.part.part) {
                            let msg = format!("Failed to remove part from item: {}", e);

                            notification =
                                Some(Notification::new(msg, NotificationSentiment::Negative));
                        }

                        item_editor_state.map_current_item_if_exists_to_editor_state();
                    }
                }
            }
            ItemEditorInteractionMessage::CurrentAnointmentPressed(current_part_type_index) => {
                let selected_item_index = item_editor_state.selected_item_index;

                if let Some(current_item) =
                    item_editor_state.items_mut().get_mut(selected_item_index)
                {
                    let part_selected = current_item
                        .editor
                        .current_parts
                        .parts
                        .get(current_part_type_index.category_index)
                        .and_then(|p| p.parts.get(current_part_type_index.part_index));

                    if let Some(part_selected) = part_selected {
                        if let Err(e) = current_item
                            .item
                            .remove_generic_part(&part_selected.part.part)
                        {
                            let msg = format!("Failed to remove part from item: {}", e);

                            notification =
                                Some(Notification::new(msg, NotificationSentiment::Negative));
                        }

                        item_editor_state.map_current_item_if_exists_to_editor_state();
                    }
                }
            }
            ItemEditorInteractionMessage::ImportItem(s) => {
                item_editor_state.import_serial_input = s;
            }
            ItemEditorInteractionMessage::CreateItemPressed => {
                item_editor_state
                    .add_item(Bl3Item::from_serial_base64("BL3(BAAAAAD2aoA+P1vAEgA=)").unwrap());

                item_editor_state.selected_item_index = item_editor_state.items().len() - 1;

                item_editor_state.map_current_item_if_exists_to_editor_state();

                item_editor_state.item_list_scrollable_state.snap_to(1.0)
            }
            ItemEditorInteractionMessage::ImportItemFromSerialPressed => {
                let item_serial = item_editor_state.import_serial_input.trim();

                match Bl3Item::from_serial_base64(item_serial) {
                    Ok(bl3_item) => {
                        item_editor_state.add_item(bl3_item);

                        item_editor_state.selected_item_index = item_editor_state.items().len() - 1;

                        item_editor_state.map_current_item_if_exists_to_editor_state();

                        item_editor_state.item_list_scrollable_state.snap_to(1.0);
                    }
                    Err(e) => {
                        let msg = format!("Failed to import serial: {}", e);

                        notification =
                            Some(Notification::new(msg, NotificationSentiment::Negative));
                    }
                }
            }
            ItemEditorInteractionMessage::AllItemLevel(item_level_input) => {
                item_editor_state.all_item_levels_input = item_level_input;

                command = Some(Command::perform(
                    async {
                        //Debounce
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    },
                    |_| ItemEditorInteractionMessage::AllItemLevelDebounced,
                ));
            }
            ItemEditorInteractionMessage::AllItemLevelDebounced => {
                let item_level = item_editor_state.all_item_levels_input as usize;

                let mut error_notification = None;

                for (i, item) in item_editor_state.items_mut().iter_mut().enumerate() {
                    if let Err(e) = item.item.set_level(item_level) {
                        let msg = format!("Failed to set level for item number: {} - {}", i, e);

                        error_notification =
                            Some(Notification::new(msg, NotificationSentiment::Negative));

                        break;
                    }
                }

                if let Some(error_notification) = error_notification {
                    notification = Some(error_notification)
                }

                item_editor_state.map_current_item_if_exists_to_editor_state();
            }
            ItemEditorInteractionMessage::ItemLevel(item_level_input) => {
                if let Err(e) = item_editor_state.map_current_item_if_exists_result(|i| {
                    i.item.set_level(item_level_input as usize)
                }) {
                    let msg = format!("Failed to set level for item: {}", e);

                    notification = Some(Notification::new(msg, NotificationSentiment::Negative));
                }
            }
            ItemEditorInteractionMessage::DeleteItem(id) => {
                item_editor_state.remove_item(id);

                match item_editor_file_type {
                    ItemEditorFileType::Save(s) => s.character_data.remove_inventory_item(id),
                    ItemEditorFileType::ProfileBank(p) => p.profile_data.remove_bank_item(id),
                }

                if item_editor_state.selected_item_index != 0 {
                    item_editor_state.selected_item_index -= 1;
                }

                item_editor_state.map_current_item_if_exists_to_editor_state();
            }
            ItemEditorInteractionMessage::DuplicateItem(id) => {
                match item_editor_state.items.get(id) {
                    Some(item) => {
                        let item = item.item.clone();

                        item_editor_state.add_item(item);

                        item_editor_state.selected_item_index = item_editor_state.items().len() - 1;

                        item_editor_state.map_current_item_if_exists_to_editor_state();

                        item_editor_state.item_list_scrollable_state.snap_to(1.0);
                    }
                    None => {
                        let msg = format!("Failed to duplicate item number {}: could not find this item to duplicate.", id);

                        notification =
                            Some(Notification::new(msg, NotificationSentiment::Negative));
                    }
                }
            }
            ItemEditorInteractionMessage::BalanceInputSelected(balance_selected) => {
                if balance_selected.ident != NO_SEARCH_RESULTS_FOUND_MESSAGE {
                    if let Err(e) = item_editor_state
                        .map_current_item_if_exists_result(|i| i.item.set_balance(balance_selected))
                    {
                        let msg = format!("Failed to set balance for item: {}", e);

                        notification =
                            Some(Notification::new(msg, NotificationSentiment::Negative));
                    } else {
                        item_editor_state
                            .map_current_item_if_exists(|i| i.editor.balance_search_input.clear())
                    }
                }
            }
            ItemEditorInteractionMessage::BalanceSearchInputChanged(balance_search_query) => {
                item_editor_state.map_current_item_if_exists(|i| {
                    i.editor.balance_search_input = balance_search_query.to_lowercase()
                });
            }
            ItemEditorInteractionMessage::InvDataInputSelected(inv_data_selected) => {
                if inv_data_selected.ident != NO_SEARCH_RESULTS_FOUND_MESSAGE {
                    if let Err(e) = item_editor_state.map_current_item_if_exists_result(|i| {
                        i.item.set_inv_data(inv_data_selected)
                    }) {
                        let msg = format!("Failed to set inventory data for item: {}", e);

                        notification =
                            Some(Notification::new(msg, NotificationSentiment::Negative));
                    } else {
                        item_editor_state
                            .map_current_item_if_exists(|i| i.editor.inv_data_search_input.clear())
                    }
                }
            }
            ItemEditorInteractionMessage::InvDataSearchInputChanged(inv_data_search_query) => {
                item_editor_state.map_current_item_if_exists(|i| {
                    i.editor.inv_data_search_input = inv_data_search_query.to_lowercase()
                });
            }
            ItemEditorInteractionMessage::ManufacturerInputSelected(manufacturer_selected) => {
                if manufacturer_selected.ident != NO_SEARCH_RESULTS_FOUND_MESSAGE {
                    if let Err(e) = item_editor_state.map_current_item_if_exists_result(|i| {
                        i.item.set_manufacturer(manufacturer_selected)
                    }) {
                        let msg = format!("Failed to set manufacturer for item: {}", e);

                        notification =
                            Some(Notification::new(msg, NotificationSentiment::Negative));
                    } else {
                        item_editor_state.map_current_item_if_exists(|i| {
                            i.editor.manufacturer_search_input.clear()
                        })
                    }
                }
            }
            ItemEditorInteractionMessage::ManufacturerSearchInputChanged(
                manufacturer_search_query,
            ) => {
                item_editor_state.map_current_item_if_exists(|i| {
                    i.editor.manufacturer_search_input = manufacturer_search_query.to_lowercase()
                });
            }
        }

        ItemEditorInteractionResponse {
            notification,
            command,
        }
    }
}

pub fn view<F>(
    item_editor_state: &mut ItemEditorState,
    interaction_message: F,
) -> Container<Bl3Message>
where
    F: Fn(ItemEditorInteractionMessage) -> InteractionMessage + 'static + Copy,
{
    let selected_item_index = item_editor_state.selected_item_index;
    let number_of_items = item_editor_state.items.len();

    let serial_importer = Row::new()
        .push(
            LabelledElement::create(
                "Import Serial",
                Length::Units(120),
                TextInputLimited::new(
                    &mut item_editor_state.import_serial_input_state,
                    "BL3(AwAAAABmboC7I9xAEzwShMJVX8nPYwsAAA==)",
                    &item_editor_state.import_serial_input,
                    500,
                    move |s| interaction_message(ItemEditorInteractionMessage::ImportItem(s)),
                )
                .0
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
                &mut item_editor_state.import_serial_button_state,
                Text::new("Import").font(JETBRAINS_MONO_BOLD).size(17),
            )
            .on_press(interaction_message(
                ItemEditorInteractionMessage::ImportItemFromSerialPressed,
            ))
            .padding(10)
            .style(Bl3UiStyle)
            .into_element(),
        )
        .align_items(Align::Center);

    let create_item_button = Container::new(
        Button::new(
            &mut item_editor_state.create_item_button_state,
            Text::new("Create Item").font(JETBRAINS_MONO_BOLD).size(17),
        )
        .on_press(interaction_message(
            ItemEditorInteractionMessage::CreateItemPressed,
        ))
        .padding(10)
        .style(Bl3UiStyle)
        .into_element(),
    );

    let edit_all_item_levels_input = Row::new()
        .push(
            LabelledElement::create(
                "All Levels",
                Length::Units(95),
                Tooltip::new(
                    NumberInput::new(
                        &mut item_editor_state.all_item_levels_input_state,
                        item_editor_state.all_item_levels_input,
                        1,
                        Some(MAX_CHARACTER_LEVEL as i32),
                        move |v| interaction_message(ItemEditorInteractionMessage::AllItemLevel(v)),
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                    format!("Level must be between 1 and {}", MAX_CHARACTER_LEVEL),
                    tooltip::Position::Top,
                )
                .gap(10)
                .padding(10)
                .font(JETBRAINS_MONO)
                .size(17)
                .style(Bl3UiTooltipStyle),
            )
            .spacing(15)
            .width(Length::FillPortion(9))
            .align_items(Align::Center),
        )
        .align_items(Align::Center);

    let general_options_row = Row::new()
        .push(create_item_button)
        .push(
            Container::new(serial_importer)
                .width(Length::FillPortion(8))
                .height(Length::Units(36))
                .style(Bl3UiStyle),
        )
        .push(
            Container::new(edit_all_item_levels_input)
                .width(Length::FillPortion(2))
                .height(Length::Units(36))
                .style(Bl3UiStyle),
        )
        .spacing(20);

    let mut item_editor = None;

    let inventory_items = item_editor_state.items.iter_mut().enumerate().fold(
        Column::new().align_items(Align::Start),
        |inventory_items, (i, item)| {
            let is_active = i == selected_item_index;

            let (list_item_button, curr_item_editor) = item.view(is_active, interaction_message);

            if is_active {
                item_editor = curr_item_editor;
            }

            inventory_items.push(list_item_button)
        },
    );

    let item_editor = if let Some(item_editor) = item_editor {
        item_editor
    } else {
        Container::new(
            Text::new("Please Import/Create an item to get started.")
                .font(JETBRAINS_MONO_BOLD)
                .size(17)
                .color(Color::from_rgb8(220, 220, 220)),
        )
        .align_x(Align::Center)
        .align_y(Align::Center)
        .style(Bl3UiStyle)
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
                    Scrollable::new(&mut item_editor_state.item_list_scrollable_state)
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
