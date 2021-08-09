use iced::{
    text_input, text_input_with_picklist, tooltip, Align, Column, Container, Length, Row,
    TextInput, TextInputWithPickList, Tooltip,
};

use bl3_save_edit_core::bl3_save::bl3_item::Bl3Item;
use bl3_save_edit_core::game_data::{GameDataKv, BALANCE_NAME_MAPPING};
use bl3_save_edit_core::resources::INVENTORY_PARTS;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::character::MAX_CHARACTER_LEVEL;
use crate::views::manage_save::inventory::available_parts::AvailableParts;
use crate::views::manage_save::inventory::current_parts::CurrentParts;
use crate::views::manage_save::inventory::InventoryInteractionMessage;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

#[derive(Debug, Default)]
pub struct InventoryItemEditor {
    pub item_level_input: i32,
    pub item_level_input_state: text_input::State,
    pub balance_input: String,
    pub balance_input_state: text_input_with_picklist::State<GameDataKv>,
    pub balance_input_selected: GameDataKv,
    pub inventory_data_input: String,
    pub inventory_data_input_state: text_input::State,
    pub manufacturer_input: String,
    pub manufacturer_input_state: text_input::State,
    pub available_parts: AvailableParts,
    pub current_parts: CurrentParts,
}

impl InventoryItemEditor {
    pub fn view(&mut self, item: &Bl3Item) -> Container<Message> {
        let item_part_data = &INVENTORY_PARTS.inventory_parts_all;
        let resource_item = item
            .balance_part
            .short_ident
            .as_ref()
            .and_then(|i| item_part_data.get(i));

        let item_editor_contents = Column::new()
            .push(
                Container::new(
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
                    .align_items(Align::Center),
                )
                .width(Length::Fill)
                .height(Length::Units(36))
                .style(Bl3UiStyle),
            )
            .push(
                Container::new(
                    LabelledElement::create(
                        "Balance",
                        Length::Units(130),
                        TextInputWithPickList::new(
                            &mut self.balance_input_state,
                            "",
                            &self.balance_input,
                            Some(self.balance_input_selected),
                            &BALANCE_NAME_MAPPING[..],
                            |s| {
                                InteractionMessage::ManageSaveInteraction(
                                    ManageSaveInteractionMessage::Inventory(
                                        InventoryInteractionMessage::BalanceInputChanged(s),
                                    ),
                                )
                            },
                            |s| {
                                InteractionMessage::ManageSaveInteraction(
                                    ManageSaveInteractionMessage::Inventory(
                                        InventoryInteractionMessage::BalanceInputSelected(s),
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
                .style(Bl3UiStyle),
            )
            .push(
                Container::new(
                    LabelledElement::create(
                        "Inventory Data",
                        Length::Units(130),
                        TextInput::new(
                            &mut self.inventory_data_input_state,
                            "",
                            &self.inventory_data_input,
                            |s| {
                                InteractionMessage::ManageSaveInteraction(
                                    ManageSaveInteractionMessage::Inventory(
                                        InventoryInteractionMessage::InventoryDataInputChanged(s),
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
                .style(Bl3UiStyle),
            )
            .push(
                Container::new(
                    LabelledElement::create(
                        "Manufacturer",
                        Length::Units(130),
                        TextInput::new(
                            &mut self.manufacturer_input_state,
                            "",
                            &self.manufacturer_input,
                            |s| {
                                InteractionMessage::ManageSaveInteraction(
                                    ManageSaveInteractionMessage::Inventory(
                                        InventoryInteractionMessage::ManufacturerInputChanged(s),
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
