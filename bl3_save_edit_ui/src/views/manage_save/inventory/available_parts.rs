use iced::{
    button, scrollable, Align, Button, Color, Column, Container, Element, Length, Scrollable, Text,
};

use bl3_save_edit_core::resources::{ResourceItem, ResourcePart};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::inventory::inventory_button_style::InventoryButtonStyle;
use crate::views::manage_save::inventory::inventory_category_style::InventoryCategoryStyle;
use crate::views::manage_save::inventory::InventoryInteractionMessage;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::text_margin::TextMargin;

#[derive(Debug, Copy, Clone, Default)]
pub struct AvailablePartsIndex {
    pub category_index: usize,
    pub part_index: usize,
}

#[derive(Debug, Clone)]
pub struct AvailableCategorizedPart {
    pub category: String,
    pub parts: Vec<AvailableResourcePart>,
}

impl AvailableCategorizedPart {
    pub fn new(category_id: usize, category: String, parts: Vec<ResourcePart>) -> Self {
        let parts = parts
            .into_iter()
            .enumerate()
            .map(|(id, p)| AvailableResourcePart::new(category_id, id, p))
            .collect();

        Self { category, parts }
    }
}

#[derive(Debug, Clone)]
pub struct AvailableResourcePart {
    category_index: usize,
    part_index: usize,
    pub part: ResourcePart,
    button_state: button::State,
}

impl AvailableResourcePart {
    pub fn new(category_index: usize, part_index: usize, part: ResourcePart) -> Self {
        Self {
            category_index,
            part_index,
            part,
            button_state: button::State::new(),
        }
    }

    pub fn view(&mut self, is_active: bool) -> Element<Message> {
        Button::new(
            &mut self.button_state,
            TextMargin::new(&self.part.name, 2)
                .0
                .font(JETBRAINS_MONO)
                .size(16),
        )
        .on_press(InteractionMessage::ManageSaveInteraction(
            ManageSaveInteractionMessage::Inventory(
                InventoryInteractionMessage::AvailablePartPressed(AvailablePartsIndex {
                    category_index: self.category_index,
                    part_index: self.part_index,
                }),
            ),
        ))
        .padding(10)
        .width(Length::Fill)
        .style(InventoryButtonStyle { is_active })
        .into_element()
    }
}

#[derive(Debug, Default)]
pub struct AvailableParts {
    pub scrollable_state: scrollable::State,
    pub parts_index: AvailablePartsIndex,
    pub parts: Vec<AvailableCategorizedPart>,
}

impl AvailableParts {
    pub fn view(&mut self, resource_item: Option<&ResourceItem>) -> Container<Message> {
        let selected_available_parts_index = &self.parts_index;

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

        if let Some(resource_item) = resource_item {
            self.parts = resource_item
                .inventory_categorized_parts
                .iter()
                .cloned()
                .enumerate()
                .map(|(cat_id, cat_p)| {
                    AvailableCategorizedPart::new(cat_id, cat_p.category, cat_p.parts)
                })
                .collect();

            let available_parts_list = self.parts.iter_mut().enumerate().fold(
                Column::new(),
                |mut curr, (cat_index, cat_parts)| {
                    curr = curr.push(
                        Container::new(
                            Text::new(&cat_parts.category)
                                .font(JETBRAINS_MONO_BOLD)
                                .size(17)
                                .color(Color::from_rgb8(242, 203, 5)),
                        )
                        .width(Length::Fill)
                        .style(InventoryCategoryStyle)
                        .padding(10),
                    );

                    for (part_index, p) in cat_parts.parts.iter_mut().enumerate() {
                        let is_active = selected_available_parts_index.category_index == cat_index
                            && selected_available_parts_index.part_index == part_index;
                        curr = curr.push(p.view(is_active));
                    }

                    curr
                },
            );

            available_parts_column = available_parts_column.push(
                Container::new(
                    Scrollable::new(&mut self.scrollable_state)
                        .push(available_parts_list)
                        .height(Length::Fill)
                        .width(Length::Fill),
                )
                .padding(1),
            );
        } else {
            available_parts_column = available_parts_column.push(
                Container::new(
                    Text::new("No available parts found.")
                        .font(JETBRAINS_MONO)
                        .size(17)
                        .color(Color::from_rgb8(220, 220, 220)),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Align::Center)
                .align_y(Align::Center),
            )
        }

        Container::new(available_parts_column)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .style(Bl3UiStyle)
    }
}
