use std::collections::BTreeMap;

use iced::{
    button, scrollable, Align, Button, Color, Column, Container, Element, Length, Scrollable, Text,
};

use bl3_save_edit_core::bl3_save::bl3_serial::Part;
use bl3_save_edit_core::resources::ResourceItem;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::inventory::{
    InventoryButtonStyle, InventoryInteractionMessage, InventoryItem,
};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::text_margin::TextMargin;

#[derive(Debug, Copy, Clone, Default)]
pub struct CurrentPartsIndex {
    pub category_index: usize,
    pub part_index: usize,
}

#[derive(Debug)]
pub struct CurrentCategorizedPart {
    pub category: String,
    pub parts: Vec<CurrentInventoryPart>,
}

impl CurrentCategorizedPart {
    pub fn new(category_id: usize, category: String, parts: Vec<Part>) -> Self {
        let parts = parts
            .into_iter()
            .enumerate()
            .map(|(id, p)| CurrentInventoryPart::new(category_id, id, p))
            .collect();

        Self { category, parts }
    }
}

#[derive(Debug)]
pub struct CurrentInventoryPart {
    category_index: usize,
    part_index: usize,
    part: Part,
    button_state: button::State,
}

impl CurrentInventoryPart {
    pub fn new(category_index: usize, part_index: usize, part: Part) -> Self {
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
            TextMargin::new(
                self.part.short_ident.as_ref().unwrap_or(&self.part.ident),
                2,
            )
            .0
            .font(JETBRAINS_MONO)
            .size(16),
        )
        .on_press(InteractionMessage::ManageSaveInteraction(
            ManageSaveInteractionMessage::Inventory(
                InventoryInteractionMessage::CurrentPartPressed(CurrentPartsIndex {
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

#[derive(Debug, Clone)]
pub struct InventoryCategorizedParts {
    pub category: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Default)]
pub struct CurrentParts {
    pub scrollable_state: scrollable::State,
    pub parts_index: CurrentPartsIndex,
    pub parts: Vec<CurrentCategorizedPart>,
}

impl CurrentParts {
    pub fn view(
        &mut self,
        active_item: Option<&InventoryItem>,
        available_parts: Option<&ResourceItem>,
    ) -> Container<Message> {
        let selected_current_parts_index = &self.parts_index;

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
            let mut categorized_parts: BTreeMap<String, Vec<Part>> = BTreeMap::new();

            if let Some(available_parts) = available_parts {
                active_item.item.parts.iter().for_each(|p| {
                    let known_cat_p =
                        available_parts
                            .inventory_categorized_parts
                            .iter()
                            .find(|cat| {
                                cat.parts.iter().any(|cat_p| {
                                    part_contains(p.short_ident.as_ref(), &p.ident, &cat_p.name)
                                })
                            });

                    if let Some(known_cat_p) = known_cat_p {
                        let curr_cat_parts = categorized_parts
                            .entry(known_cat_p.category.to_owned())
                            .or_insert_with(Vec::new);

                        curr_cat_parts.push(p.to_owned());
                    } else {
                        let curr_cat_parts = categorized_parts
                            .entry("UNKNOWN PARTS".to_owned())
                            .or_insert_with(Vec::new);

                        curr_cat_parts.push(p.to_owned());
                    }
                });
            } else {
                active_item.item.parts.iter().for_each(|p| {
                    let curr_cat_parts = categorized_parts
                        .entry("UNKNOWN PARTS".to_owned())
                        .or_insert_with(Vec::new);

                    curr_cat_parts.push(p.to_owned());
                })
            }

            let inventory_categorized_parts =
                categorized_parts.into_iter().map(|(category, mut parts)| {
                    parts.sort();
                    InventoryCategorizedParts { category, parts }
                });

            self.parts = inventory_categorized_parts
                .into_iter()
                .enumerate()
                .map(|(cat_id, cat_p)| {
                    CurrentCategorizedPart::new(cat_id, cat_p.category, cat_p.parts)
                })
                .collect();

            let current_parts_list = self.parts.iter_mut().enumerate().fold(
                Column::new(),
                |mut curr, (cat_index, cat_parts)| {
                    curr = curr.push(
                        Container::new(
                            Text::new(&cat_parts.category)
                                .font(JETBRAINS_MONO_BOLD)
                                .size(17)
                                .color(Color::from_rgb8(242, 203, 5)),
                        )
                        .padding(10),
                    );

                    for (part_index, p) in cat_parts.parts.iter_mut().enumerate() {
                        let is_active = selected_current_parts_index.category_index == cat_index
                            && selected_current_parts_index.part_index == part_index;
                        curr = curr.push(p.view(is_active))
                    }

                    curr
                },
            );

            current_parts_column = current_parts_column.push(
                Container::new(
                    Scrollable::new(&mut self.scrollable_state)
                        .push(current_parts_list)
                        .height(Length::Fill)
                        .width(Length::Fill),
                )
                .padding(1),
            );
        } else {
            current_parts_column = current_parts_column.push(
                Container::new(
                    Text::new("No current parts found.")
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

        Container::new(current_parts_column)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .style(Bl3UiStyle)
    }
}

fn part_contains(short_ident: Option<&String>, ident: &str, cat_part_name: &str) -> bool {
    if let Some(short_ident) = short_ident {
        cat_part_name.to_lowercase() == short_ident.to_lowercase()
    } else {
        ident.to_lowercase().contains(&cat_part_name.to_lowercase())
    }
}
