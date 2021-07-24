use iced::{
    button, scrollable, text_input, Align, Button, Color, Column, Container, Element, Length, Row,
    Scrollable, Text, TextInput,
};

use bl3_save_edit_core::bl3_save::bl3_serial::Bl3Serial;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::interaction::InteractionExt;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::text_margin::TextMargin;

#[derive(Debug, Default)]
pub struct InventoryState {
    pub items: Vec<InventoryItem>,
    pub scrollable_state: scrollable::State,
    pub balance_input: String,
    pub balance_input_state: text_input::State,
}

#[derive(Debug)]
pub enum InventoryMessage {}

#[derive(Debug, Clone)]
pub enum InventoryInteractionMessage {
    ItemPressed(usize),
    BalanceInputChanged(String),
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
                .style(Bl3UiStyle),
            ),
    )
    .width(Length::FillPortion(3));

    //Todo: Each item should have it's own state

    let item_editor = Container::new(
        Column::new()
            .push(
                Container::new(
                    TextMargin::new("Item Data", 2)
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
                    LabelledElement::create(
                        "Balance",
                        Length::Units(90),
                        TextInput::new(
                            &mut inventory_state.balance_input_state,
                            "",
                            &inventory_state.balance_input,
                            |s| {
                                InteractionMessage::ManageSaveInteraction(
                                    ManageSaveInteractionMessage::Inventory(
                                        InventoryInteractionMessage::BalanceInputChanged(s),
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
            ),
    )
    .width(Length::FillPortion(7));

    let all_contents = Row::new().push(item_list).push(item_editor).spacing(20);

    Container::new(all_contents).padding(30)
}
