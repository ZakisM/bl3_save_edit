use std::borrow::Cow;

use iced::{
    button, scrollable, text_input, text_input_with_picklist, Align, Button, Color, Column,
    Container, Element, Length, Row, Scrollable, Text, TextInput, TextInputWithPickList,
};

use bl3_save_edit_core::bl3_save::bl3_serial::Bl3Serial;
use bl3_save_edit_core::game_data::{GameDataKv, BALANCE_NAME_MAPPING};
use bl3_save_edit_core::resources::INVENTORY_PARTS_SHIELDS;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::text_margin::TextMargin;

#[derive(Debug, Default)]
pub struct InventoryState {
    pub items: Vec<InventoryItem>,
    pub scrollable_state: scrollable::State,
    pub balance_input: String,
    pub balance_input_state: text_input_with_picklist::State<GameDataKv>,
    pub balance_input_selected: GameDataKv,
    pub inventory_data_input: String,
    pub inventory_data_input_state: text_input::State,
    pub manufacturer_input: String,
    pub manufacturer_input_state: text_input::State,
}

#[derive(Debug)]
pub enum InventoryMessage {}

#[derive(Debug, Clone)]
pub enum InventoryInteractionMessage {
    ItemPressed(usize),
    BalanceInputChanged(String),
    BalanceInputSelected(GameDataKv),
    InventoryDataInputChanged(String),
    ManufacturerInputChanged(String),
}

#[derive(Debug)]
pub struct InventoryItem {
    id: usize,
    item: Bl3Serial,
    label: String,
    item_state: button::State,
    pub is_active: bool,
}

impl InventoryItem {
    pub fn new(id: usize, item: Bl3Serial, is_active: bool) -> Self {
        let balance_part = &item.balance_part;

        let label = format!(
            "[Lvl {}] {}",
            item.level,
            balance_part
                .name
                .clone()
                .unwrap_or_else(|| balance_part.ident.clone()),
        );

        InventoryItem {
            id,
            label,
            item,
            item_state: button::State::new(),
            is_active,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Button::new(
            &mut self.item_state,
            Text::new(&self.label).font(JETBRAINS_MONO).size(16),
        )
        .on_press(InteractionMessage::ManageSaveInteraction(
            ManageSaveInteractionMessage::Inventory(InventoryInteractionMessage::ItemPressed(
                self.id,
            )),
        ))
        .padding(10)
        .width(Length::Fill)
        .style(InventoryItemStyle {
            is_active: self.is_active,
        })
        .into_element()
    }
}

pub struct InventoryItemStyle {
    pub is_active: bool,
}

impl button::StyleSheet for InventoryItemStyle {
    fn active(&self) -> button::Style {
        let (background, text_color) = if self.is_active {
            (
                Some(Color::from_rgb8(35, 35, 35).into()),
                Color::from_rgb8(255, 255, 255),
            )
        } else {
            (
                Some(Color::from_rgb8(23, 23, 23).into()),
                Color::from_rgb8(220, 220, 220),
            )
        };

        button::Style {
            background,
            text_color,
            border_width: 0.0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(35, 35, 35).into()),
            border_width: 0.0,
            text_color: Color::from_rgb8(255, 255, 255),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(30, 30, 30).into()),
            border_width: 0.0,
            text_color: Color::from_rgb8(220, 220, 220),
            ..button::Style::default()
        }
    }
}

pub fn view(inventory_state: &mut InventoryState) -> Container<Message> {
    //Todo: Each item should have it's own state

    let mut item_editor_column = Column::new()
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

    let active_item = inventory_state.items.iter().find(|i| i.is_active);

    let inv_part_shields = &*INVENTORY_PARTS_SHIELDS;

    if let Some(active_item_parts) =
        active_item.and_then(|i| inv_part_shields.get(&Cow::from(&i.item.balance_part.short_name)))
    {
        let parts_column = active_item_parts.inventory_categorized_parts.iter().fold(
            Column::new(),
            |curr, item| {
                let item_sub_parts =
                    item.parts
                        .iter()
                        .fold(Column::new(), |sub_part_curr, sub_part| {
                            sub_part_curr.push(Text::new(format!("{}", sub_part.name)))
                        });

                curr.push(
                    Column::new()
                        .push(Text::new(format!("{}", item.category)))
                        .push(item_sub_parts),
                )
            },
        );

        item_editor_column = item_editor_column.push(parts_column);
    }

    let inventory_items = inventory_state
        .items
        .iter_mut()
        .fold(Column::new().align_items(Align::Start), |curr, item| {
            curr.push(item.view())
        });

    let item_list = Container::new(
        Column::new()
            .push(
                Container::new(
                    TextMargin::new("Items", 2)
                        .0
                        .font(JETBRAINS_MONO)
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
                    Scrollable::new(&mut inventory_state.scrollable_state).push(inventory_items),
                )
                .padding(1)
                .height(Length::Fill),
            ),
    )
    .width(Length::FillPortion(3))
    .height(Length::Fill)
    .style(Bl3UiStyle);

    let item_editor = Container::new(item_editor_column)
        .width(Length::FillPortion(7))
        .height(Length::Fill);

    let all_contents = Row::new().push(item_list).push(item_editor).spacing(20);

    Container::new(all_contents).padding(30)
}
