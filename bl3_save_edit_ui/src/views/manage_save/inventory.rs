use iced::{
    button, scrollable, text_input, text_input_with_picklist, Align, Button, Color, Column,
    Container, Element, Length, Row, Scrollable, Text, TextInput, TextInputWithPickList,
};

use bl3_save_edit_core::bl3_save::bl3_serial::Bl3Serial;
use bl3_save_edit_core::game_data::{GameDataKv, BALANCE_NAME_MAPPING};
use bl3_save_edit_core::resources::INVENTORY_PARTS_SHIELDS;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::text_margin::TextMargin;

#[derive(Debug, Default)]
pub struct InventoryState {
    pub selected_item_index: usize,
    pub items: Vec<InventoryItem>,
    pub item_list_scrollable_state: scrollable::State,
    pub available_parts_scrollable_state: scrollable::State,
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
}

impl InventoryItem {
    pub fn new(id: usize, item: Bl3Serial) -> Self {
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
        }
    }

    pub fn view(&mut self, is_active: bool) -> Element<Message> {
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
        .style(InventoryItemStyle { is_active })
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

    let selected_item_index = inventory_state.selected_item_index;

    let mut item_editor_contents = Column::new()
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

    let active_item = inventory_state.items.get(selected_item_index);

    dbg!(&active_item.unwrap().item);

    let inv_part_shields = &*INVENTORY_PARTS_SHIELDS;

    let mut available_parts_column = Column::new().push(
        Container::new(
            TextMargin::new("Available Parts", 2)
                .0
                .font(JETBRAINS_MONO_BOLD)
                .size(17)
                .color(Color::from_rgb8(242, 203, 5)),
        )
        .padding(10)
        .align_x(Align::Center)
        .width(Length::FillPortion(2))
        .style(Bl3UiStyle),
    );

    let mut current_parts_column = Column::new().push(
        Container::new(
            TextMargin::new("Current Parts", 2)
                .0
                .font(JETBRAINS_MONO_BOLD)
                .size(17)
                .color(Color::from_rgb8(242, 203, 5)),
        )
        .padding(10)
        .align_x(Align::Center)
        .width(Length::FillPortion(2))
        .style(Bl3UiStyle),
    );

    if let Some(active_item) = active_item {
        if let Some(available_parts) = active_item
            .item
            .balance_part
            .short_ident
            .as_ref()
            .and_then(|i| inv_part_shields.get(i))
        {
            let available_parts_list = available_parts.inventory_categorized_parts.iter().fold(
                Column::new(),
                |curr, categorized_part| {
                    let categorized_part_sub_parts = categorized_part.parts.iter().fold(
                        Column::new(),
                        |sub_part_curr, sub_part| {
                            sub_part_curr.push(
                                Text::new(format!("{}", sub_part.name))
                                    .font(JETBRAINS_MONO)
                                    .size(16)
                                    .color(Color::from_rgb8(255, 255, 255)),
                            )
                        },
                    );

                    curr.push(
                        Column::new()
                            .push(
                                Text::new(format!("{}", categorized_part.category))
                                    .font(JETBRAINS_MONO_BOLD)
                                    .size(17)
                                    .color(Color::from_rgb8(242, 203, 5)),
                            )
                            .push(categorized_part_sub_parts),
                    )
                },
            );

            available_parts_column = available_parts_column.push(
                Container::new(
                    Scrollable::new(&mut inventory_state.item_list_scrollable_state)
                        .push(available_parts_list)
                        .height(Length::Fill)
                        .width(Length::Fill),
                )
                .padding(1),
            );
        }

        let current_parts_list = active_item
            .item
            .parts
            .iter()
            .fold(Column::new(), |curr, part| {
                curr.push(
                    Text::new(
                        part.short_ident
                            .clone()
                            .unwrap_or_else(|| part.ident.clone()),
                    )
                    .font(JETBRAINS_MONO)
                    .size(16)
                    .color(Color::from_rgb8(255, 255, 255)),
                )
            });

        current_parts_column = current_parts_column.push(Container::new(current_parts_list))
    }

    let available_parts_contents = Container::new(available_parts_column)
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .style(Bl3UiStyle);

    let current_parts_contents = Container::new(current_parts_column)
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .style(Bl3UiStyle);

    let parts_editor = Row::new()
        .push(available_parts_contents)
        .push(current_parts_contents)
        .spacing(20);

    let parts_editor_contents = Container::new(parts_editor)
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
                    Scrollable::new(&mut inventory_state.available_parts_scrollable_state)
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
