use iced::{
    button, pick_list, text_input, tooltip, Align, Button, Column, Container, Length, PickList,
    Row, Text, TextInput, Tooltip,
};

use bl3_save_edit_core::bl3_save::bl3_item::{BalancePart, Bl3Item, InvDataPart, ManufacturerPart};
use bl3_save_edit_core::resources::{
    INVENTORY_BALANCE_PARTS, INVENTORY_INV_DATA_PARTS, INVENTORY_MANUFACTURER_PARTS,
    INVENTORY_PARTS_ALL_CATEGORIZED, INVENTORY_SERIAL_DB_PARTS_CATEGORIZED,
};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::item_editor::available_parts::AvailableParts;
use crate::views::item_editor::current_parts::CurrentParts;
use crate::views::item_editor::delete_item_button_style::DeleteItemButtonStyle;
use crate::views::item_editor::ItemEditorInteractionMessage;
use crate::views::manage_save::character::MAX_CHARACTER_LEVEL;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

#[derive(Debug, Default)]
pub struct Editor {
    pub item_level_input: i32,
    pub item_level_input_state: text_input::State,
    pub sync_item_level_char_level_button: button::State,
    pub serial_input: String,
    pub serial_input_state: text_input::State,
    pub delete_item_button: button::State,
    pub balance_input_state: pick_list::State<BalancePart>,
    pub balance_input_selected: BalancePart,
    pub inv_data_input_state: pick_list::State<InvDataPart>,
    pub inv_data_input_selected: InvDataPart,
    pub manufacturer_input_state: pick_list::State<ManufacturerPart>,
    pub manufacturer_input_selected: ManufacturerPart,
    pub available_parts: AvailableParts,
    pub current_parts: CurrentParts,
}

impl Editor {
    pub fn view<F>(
        &mut self,
        item_id: usize,
        item: &Bl3Item,
        interaction_message: F,
    ) -> Container<Message>
    where
        F: Fn(ItemEditorInteractionMessage) -> InteractionMessage + 'static + Copy,
    {
        let inventory_serial_db_parts_categorized = &*INVENTORY_SERIAL_DB_PARTS_CATEGORIZED;
        let inventory_parts_all_categorized = &INVENTORY_PARTS_ALL_CATEGORIZED;

        let anointments_list = inventory_serial_db_parts_categorized
            .get("InventoryGenericPartData")
            .expect("Missing generic part data.");

        let specific_parts_list = item
            .balance_part()
            .short_ident
            .as_ref()
            .and_then(|i| inventory_parts_all_categorized.get(i))
            .map(|i| &i.inventory_categorized_parts);

        let all_parts_list = item
            .item_parts
            .as_ref()
            .map(|ip| ip.part_inv_key.as_str())
            .and_then(|p| inventory_serial_db_parts_categorized.get(p));

        let item_level_editor = Row::new()
            .push(
                LabelledElement::create(
                    "Level",
                    Length::Units(60),
                    Tooltip::new(
                        NumberInput::new(
                            &mut self.item_level_input_state,
                            self.item_level_input,
                            1,
                            Some(MAX_CHARACTER_LEVEL as i32),
                            move |v| {
                                interaction_message(ItemEditorInteractionMessage::ItemLevel(v))
                            },
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

        let level_serial_delete_row = Row::new()
            .push(
                Container::new(item_level_editor)
                    .width(Length::Fill)
                    .height(Length::Units(36))
                    .style(Bl3UiStyle),
            )
            .push(
                Container::new(
                    LabelledElement::create(
                        "Serial",
                        Length::Units(85),
                        TextInput::new(
                            &mut self.serial_input_state,
                            "BL3(AwAAAABmboC7I9xAEzwShMJVX8nPYwsAAA==)",
                            &self.serial_input,
                            |_| InteractionMessage::Ignore,
                        )
                        .font(JETBRAINS_MONO)
                        .padding(10)
                        .size(17)
                        .style(Bl3UiStyle)
                        .into_element(),
                    )
                    .align_items(Align::Center),
                )
                .width(Length::Fill)
                .height(Length::Units(36))
                .style(Bl3UiStyle),
            )
            .push(
                Button::new(
                    &mut self.delete_item_button,
                    Text::new("Delete Item").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(interaction_message(
                    ItemEditorInteractionMessage::DeleteItem(item_id),
                ))
                .padding(10)
                .style(DeleteItemButtonStyle)
                .into_element(),
            )
            .spacing(20);

        let item_editor_contents = Column::new()
            .push(level_serial_delete_row)
            .push(
                Container::new(
                    LabelledElement::create(
                        "Balance",
                        Length::Units(130),
                        PickList::new(
                            &mut self.balance_input_state,
                            &INVENTORY_BALANCE_PARTS[..],
                            Some(self.balance_input_selected.clone()),
                            move |s| {
                                interaction_message(
                                    ItemEditorInteractionMessage::BalanceInputSelected(s),
                                )
                            },
                        )
                        .font(JETBRAINS_MONO)
                        .text_size(16)
                        .padding(10)
                        .style(Bl3UiStyle)
                        .width(Length::Fill)
                        .into_element(),
                    )
                    .spacing(15)
                    .width(Length::Fill)
                    .align_items(Align::Center),
                )
                .style(Bl3UiStyle),
            )
            .push(
                Container::new(
                    LabelledElement::create(
                        "Inventory Data",
                        Length::Units(130),
                        PickList::new(
                            &mut self.inv_data_input_state,
                            &INVENTORY_INV_DATA_PARTS[..],
                            Some(self.inv_data_input_selected.clone()),
                            move |s| {
                                // InteractionMessage::ManageSaveInteraction(
                                //     ManageSaveInteractionMessage::Inventory(
                                //         SaveInventoryInteractionMessage::InvDataInputSelected(s),
                                //     ),
                                // )
                                interaction_message(
                                    ItemEditorInteractionMessage::InvDataInputSelected(s),
                                )
                            },
                        )
                        .font(JETBRAINS_MONO)
                        .text_size(16)
                        .padding(10)
                        .style(Bl3UiStyle)
                        .width(Length::Fill)
                        .into_element(),
                    )
                    .spacing(15)
                    .width(Length::FillPortion(9))
                    .align_items(Align::Center),
                )
                .style(Bl3UiStyle),
            )
            .push(
                Container::new(
                    LabelledElement::create(
                        "Manufacturer",
                        Length::Units(130),
                        PickList::new(
                            &mut self.manufacturer_input_state,
                            &INVENTORY_MANUFACTURER_PARTS[..],
                            Some(self.manufacturer_input_selected.clone()),
                            move |s| {
                                interaction_message(
                                    ItemEditorInteractionMessage::ManufacturerInputSelected(s),
                                )
                            },
                        )
                        .font(JETBRAINS_MONO)
                        .text_size(16)
                        .padding(10)
                        .style(Bl3UiStyle)
                        .width(Length::Fill)
                        .into_element(),
                    )
                    .spacing(15)
                    .width(Length::FillPortion(9))
                    .align_items(Align::Center),
                )
                .style(Bl3UiStyle),
            )
            .spacing(20);

        let available_parts_contents = self.available_parts.view(
            anointments_list,
            specific_parts_list,
            all_parts_list,
            interaction_message,
        );

        let current_parts_contents =
            self.current_parts
                .view(item, anointments_list, all_parts_list, interaction_message);

        let parts_editor_contents = Container::new(
            Row::new()
                .push(available_parts_contents)
                .push(current_parts_contents)
                .spacing(20),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let item_editor_contents = item_editor_contents.push(parts_editor_contents);

        Container::new(item_editor_contents)
    }
}
