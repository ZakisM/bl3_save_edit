use iced::{
    button, pick_list, text_input, tooltip, Align, Button, Column, Container, Length, PickList,
    Row, Text, TextInput, Tooltip,
};

use bl3_save_edit_core::bl3_save::bl3_item::{BalancePart, Bl3Item, InvDataPart, ManufacturerPart};
use bl3_save_edit_core::resources::{
    INVENTORY_BALANCE_PARTS, INVENTORY_INV_DATA_PARTS, INVENTORY_MANUFACTURER_PARTS,
    INVENTORY_PARTS_ALL_CATEGORIZED,
};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::character::MAX_CHARACTER_LEVEL;
use crate::views::manage_save::inventory::available_parts::AvailableParts;
use crate::views::manage_save::inventory::current_parts::CurrentParts;
use crate::views::manage_save::inventory::delete_item_button_style::DeleteItemButtonStyle;
use crate::views::manage_save::inventory::InventoryInteractionMessage;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

#[derive(Debug, Default)]
pub struct InventoryItemEditor {
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

impl InventoryItemEditor {
    pub fn view(&mut self, item_id: usize, item: &Bl3Item) -> Container<Message> {
        let item_part_data = &INVENTORY_PARTS_ALL_CATEGORIZED;

        let resource_item = item
            .balance_part()
            .short_ident
            .as_ref()
            .and_then(|i| item_part_data.get(i));

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
                            |v| {
                                InteractionMessage::ManageSaveInteraction(
                                    ManageSaveInteractionMessage::Inventory(
                                        InventoryInteractionMessage::ItemLevelInputChanged(v),
                                    ),
                                )
                            },
                        )
                        .0
                        .font(JETBRAINS_MONO)
                        .padding(10)
                        .size(17)
                        .style(Bl3UiStyle)
                        .into_element(),
                        "Level must be between 1 and 72",
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
            .push(
                Tooltip::new(
                    Button::new(
                        &mut self.sync_item_level_char_level_button,
                        Text::new("Sync").font(JETBRAINS_MONO_BOLD).size(17),
                    )
                    .on_press(InteractionMessage::ManageSaveInteraction(
                        ManageSaveInteractionMessage::Inventory(
                            InventoryInteractionMessage::SyncItemLevelWithCharacterLevel,
                        ),
                    ))
                    .padding(10)
                    .style(Bl3UiStyle)
                    .into_element(),
                    "Synchronize this item level with your Character level",
                    tooltip::Position::Top,
                )
                .gap(10)
                .padding(10)
                .font(JETBRAINS_MONO)
                .size(17)
                .style(Bl3UiTooltipStyle),
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
                .on_press(InteractionMessage::ManageSaveInteraction(
                    ManageSaveInteractionMessage::Inventory(
                        InventoryInteractionMessage::DeleteItem(item_id),
                    ),
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
                            |s| {
                                InteractionMessage::ManageSaveInteraction(
                                    ManageSaveInteractionMessage::Inventory(
                                        InventoryInteractionMessage::BalanceInputSelected(s),
                                    ),
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
                            |s| {
                                InteractionMessage::ManageSaveInteraction(
                                    ManageSaveInteractionMessage::Inventory(
                                        InventoryInteractionMessage::InvDataInputSelected(s),
                                    ),
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
                            |s| {
                                InteractionMessage::ManageSaveInteraction(
                                    ManageSaveInteractionMessage::Inventory(
                                        InventoryInteractionMessage::ManufacturerInputSelected(s),
                                    ),
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

        let available_parts_contents = self.available_parts.view(resource_item);

        let current_parts_contents = self.current_parts.view(item, resource_item);

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