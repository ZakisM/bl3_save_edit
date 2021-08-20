use iced::{
    button, scrollable, Align, Button, Checkbox, Color, Column, Container, Element, Length, Row,
    Scrollable, Text,
};

use bl3_save_edit_core::resources::{ResourceCategorizedParts, ResourcePart};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::inventory::extra_part_info::add_extra_part_info;
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

    pub fn from_resource_categorized_parts(parts: &[ResourceCategorizedParts]) -> Vec<Self> {
        parts
            .iter()
            .cloned()
            .enumerate()
            .map(|(cat_id, cat_p)| {
                AvailableCategorizedPart::new(cat_id, cat_p.category, cat_p.parts)
            })
            .collect::<Vec<_>>()
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
        let part_contents_col = Column::new()
            .push(
                TextMargin::new(&self.part.name, 2)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(16),
            )
            .spacing(10);

        let part_contents_col = add_extra_part_info(part_contents_col, &self.part.info);

        let part_contents = Container::new(part_contents_col).align_x(Align::Start);

        Button::new(&mut self.button_state, part_contents)
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
    pub show_all_available_parts: bool,
}

impl AvailableParts {
    pub fn view(
        &mut self,
        specific_parts_list: Option<&Vec<ResourceCategorizedParts>>,
        all_parts_list: Option<&Vec<ResourceCategorizedParts>>,
    ) -> Container<Message> {
        let selected_available_parts_index = &self.parts_index;

        let specific_parts = specific_parts_list
            .map(|i| AvailableCategorizedPart::from_resource_categorized_parts(i));

        let all_parts =
            all_parts_list.map(|i| AvailableCategorizedPart::from_resource_categorized_parts(i));

        let mut title_row = Row::new().push(
            Container::new(
                TextMargin::new(
                    "Available Parts",
                    if specific_parts.is_some() { 8 } else { 0 },
                )
                .0
                .font(JETBRAINS_MONO_BOLD)
                .size(17)
                .color(Color::from_rgb8(242, 203, 5)),
            )
            .align_x(Align::Center)
            .width(Length::Fill),
        );

        if specific_parts.is_some() {
            title_row = title_row.push(Container::new(
                Checkbox::new(self.show_all_available_parts, "All", |c| {
                    InteractionMessage::ManageSaveInteraction(
                        ManageSaveInteractionMessage::Inventory(
                            InventoryInteractionMessage::ShowAllAvailablePartsSelected(c),
                        ),
                    )
                })
                .size(17)
                .font(JETBRAINS_MONO_BOLD)
                .text_color(Color::from_rgb8(220, 220, 220))
                .text_size(17)
                .style(Bl3UiStyle)
                .into_element(),
            ));
        }

        let available_parts = if self.show_all_available_parts || specific_parts.is_none() {
            all_parts
        } else {
            specific_parts
        };

        let mut available_parts_column =
            Column::new().push(Container::new(title_row).padding(11).style(Bl3UiStyle));

        if let Some(available_parts) = available_parts {
            self.parts = available_parts;

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
