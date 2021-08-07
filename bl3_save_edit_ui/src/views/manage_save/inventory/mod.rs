use iced::{
    scrollable, text_input, text_input_with_picklist, Align, Color, Column, Container, Length, Row,
    Scrollable, TextInput, TextInputWithPickList,
};

use bl3_save_edit_core::game_data::{GameDataKv, BALANCE_NAME_MAPPING};
use bl3_save_edit_core::resources::INVENTORY_PARTS_ALL;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::inventory::available_parts::{AvailableParts, AvailablePartsIndex};
use crate::views::manage_save::inventory::current_parts::{CurrentParts, CurrentPartsIndex};
use crate::views::manage_save::inventory::inventory_item::InventoryItem;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::text_margin::TextMargin;

pub mod available_parts;
pub mod current_parts;
pub mod inventory_button_style;
pub mod inventory_category_style;
pub mod inventory_item;

#[derive(Debug, Default)]
pub struct InventoryState {
    pub selected_item_index: usize,
    pub items: Vec<InventoryItem>,
    pub item_list_scrollable_state: scrollable::State,
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

#[derive(Debug)]
pub enum InventoryMessage {}

#[derive(Debug, Clone)]
pub enum InventoryInteractionMessage {
    ItemPressed(usize),
    AvailablePartPressed(AvailablePartsIndex),
    CurrentPartPressed(CurrentPartsIndex),
    BalanceInputChanged(String),
    BalanceInputSelected(GameDataKv),
    InventoryDataInputChanged(String),
    ManufacturerInputChanged(String),
}

pub fn view(inventory_state: &mut InventoryState) -> Container<Message> {
    //Todo: Each item should have it's own state

    let selected_item_index = inventory_state.selected_item_index;
    let item_part_data = &*INVENTORY_PARTS_ALL;
    let active_item = inventory_state.items.get(selected_item_index);

    let item_editor_contents = Column::new()
        .push(
            Container::new(
                LabelledElement::create(
                    "Balance",
                    Length::Units(130),
                    TextInputWithPickList::new(
                        &mut inventory_state.balance_input_state,
                        "",
                        &inventory_state.balance_input,
                        Some(inventory_state.balance_input_selected),
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
                        &mut inventory_state.inventory_data_input_state,
                        "",
                        &inventory_state.inventory_data_input,
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
                        &mut inventory_state.manufacturer_input_state,
                        "",
                        &inventory_state.manufacturer_input,
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

    let resource_item = active_item.and_then(|inv_item| {
        inv_item
            .item
            .balance_part
            .short_ident
            .as_ref()
            .and_then(|i| item_part_data.get(i))
    });

    let available_parts_contents = inventory_state.available_parts.view(resource_item);

    let current_parts_contents = inventory_state.current_parts.view(
        inventory_state
            .items
            .get(inventory_state.selected_item_index),
        resource_item,
    );

    let parts_editor_contents = Container::new(
        Row::new()
            .push(available_parts_contents)
            .push(current_parts_contents)
            .spacing(20),
    )
    .width(Length::Fill)
    .height(Length::Fill);

    let item_editor_contents = item_editor_contents.push(parts_editor_contents);

    let inventory_items = inventory_state.items.iter_mut().enumerate().fold(
        Column::new().align_items(Align::Start),
        |curr, (i, item)| curr.push(item.view(i == selected_item_index)),
    );

    let item_list = Container::new(
        Column::new()
            .push(
                Container::new(
                    TextMargin::new("Items", 2)
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

    let item_editor = Container::new(item_editor_contents)
        .width(Length::FillPortion(7))
        .height(Length::Fill);

    let all_contents = Row::new().push(item_list).push(item_editor).spacing(20);

    Container::new(all_contents).padding(30)
}
