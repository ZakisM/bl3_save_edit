use std::collections::BTreeMap;

use iced::alignment::{Horizontal, Vertical};
use iced::{
    button, scrollable, text_input, tooltip, Alignment, Button, Checkbox, Color, Column, Container,
    Element, Length, Row, Scrollable, Text, Tooltip,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;

use bl3_save_edit_core::bl3_item::{
    Bl3Item, Bl3Part, MAX_BL3_ITEM_ANOINTMENTS, MAX_BL3_ITEM_PARTS,
};
use bl3_save_edit_core::resources::{ResourceCategorizedParts, ResourcePart, ResourcePartInfo};

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiStyleNoBorder, Bl3UiTooltipStyle};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::item_editor::extra_part_info::add_extra_part_info;
use crate::views::item_editor::item_button_style::ItemEditorButtonStyle;
use crate::views::item_editor::parts::filter_parts;
use crate::views::item_editor::parts_tab_bar::CurrentPartType;
use crate::views::item_editor::ItemEditorInteractionMessage;
use crate::views::tab_bar_button::tab_bar_button;
use crate::views::{InteractionExt, NO_SEARCH_RESULTS_FOUND_MESSAGE};
use crate::widgets::text_input_limited::TextInputLimited;

#[derive(Debug, Copy, Clone, Default)]
pub struct CurrentPartTypeIndex {
    pub category_index: usize,
    pub part_index: usize,
}

#[derive(Debug, Clone)]
pub struct CurrentCategorizedPart {
    pub category: String,
    pub parts: Vec<CurrentItemEditorPart>,
}

impl CurrentCategorizedPart {
    pub fn new(
        category_id: usize,
        category: String,
        part_type: CurrentPartType,
        parts: Vec<Bl3PartWithInfo>,
    ) -> Self {
        let parts = parts
            .into_iter()
            .enumerate()
            .map(|(id, p)| CurrentItemEditorPart::new(category_id, id, part_type.clone(), p))
            .collect();

        Self { category, parts }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CurrentItemEditorPart {
    category_index: usize,
    part_index: usize,
    part_type: CurrentPartType,
    pub part: Bl3PartWithInfo,
    button_state: button::State,
}

impl CurrentItemEditorPart {
    pub fn new(
        category_index: usize,
        part_index: usize,
        part_type: CurrentPartType,
        part: Bl3PartWithInfo,
    ) -> Self {
        Self {
            category_index,
            part_index,
            part_type,
            part,
            button_state: button::State::new(),
        }
    }

    pub fn view<F>(
        &mut self,
        reorder_parts: bool,
        is_active: bool,
        interaction_message: F,
    ) -> Element<Bl3Message>
    where
        F: Fn(ItemEditorInteractionMessage) -> InteractionMessage + 'static + Copy,
    {
        let is_active = if reorder_parts { is_active } else { false };

        let part_contents_col = Column::new()
            .push(
                Text::new(
                    self.part
                        .part
                        .short_ident
                        .as_ref()
                        .unwrap_or(&self.part.part.ident),
                )
                .font(JETBRAINS_MONO)
                .size(16),
            )
            .spacing(10);

        let part_contents_col = add_extra_part_info(part_contents_col, &self.part.info);

        let part_contents = Container::new(part_contents_col).align_x(Horizontal::Left);

        let index = CurrentPartTypeIndex {
            category_index: self.category_index,
            part_index: self.part_index,
        };

        Button::new(&mut self.button_state, part_contents)
            .on_press(interaction_message(match self.part_type {
                CurrentPartType::Parts => {
                    ItemEditorInteractionMessage::CurrentPartPressed(reorder_parts, index)
                }
                CurrentPartType::Anointments => {
                    ItemEditorInteractionMessage::CurrentAnointmentPressed(index)
                }
            }))
            .padding(10)
            .width(Length::Fill)
            .style(ItemEditorButtonStyle { is_active })
            .into_element()
    }
}

#[derive(Debug, Clone)]
pub struct ItemEditorCategorizedParts {
    pub category: String,
    pub parts: Vec<Bl3PartWithInfo>,
}

#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Bl3PartWithInfo {
    pub part: Bl3Part,
    pub info: ResourcePartInfo,
}

#[derive(Debug, Default)]
pub struct CurrentParts {
    pub scrollable_state: scrollable::State,
    pub part_type_index: CurrentPartTypeIndex,
    pub parts: Vec<CurrentCategorizedPart>,
    pub parts_tab_type: CurrentPartType,
    pub current_parts_tab_button_state: button::State,
    pub current_anointments_tab_button_state: button::State,
    pub search_input: String,
    pub search_input_state: text_input::State,
    pub reorder_parts: bool,
    pub reorder_parts_move_up_button_state: button::State,
    pub reorder_parts_move_down_button_state: button::State,
    pub reorder_parts_move_top_button_state: button::State,
    pub reorder_parts_move_bottom_button_state: button::State,
}

impl CurrentParts {
    pub fn view<F>(
        &mut self,
        item: &Bl3Item,
        anointments_list: &[ResourceCategorizedParts],
        _specific_parts_list: Option<&Vec<ResourceCategorizedParts>>,
        all_parts_list: Option<&Vec<ResourceCategorizedParts>>,
        interaction_message: F,
    ) -> Container<Bl3Message>
    where
        F: Fn(ItemEditorInteractionMessage) -> InteractionMessage + 'static + Copy,
    {
        let selected_current_part_index = &self.part_type_index;
        let reorder_parts = self.reorder_parts;

        let title_row = Row::new()
            .push(
                Container::new(tab_bar_button(
                    &mut self.current_parts_tab_button_state,
                    CurrentPartType::Parts,
                    &self.parts_tab_type,
                    interaction_message(ItemEditorInteractionMessage::CurrentPartsTabPressed),
                    Some(format!(
                        "({}/{})",
                        item.item_parts
                            .as_ref()
                            .map(|ip| ip.parts().len())
                            .unwrap_or(0),
                        MAX_BL3_ITEM_PARTS
                    )),
                ))
                .padding(1)
                .width(Length::FillPortion(2)),
            )
            .push(
                Container::new(tab_bar_button(
                    &mut self.current_anointments_tab_button_state,
                    CurrentPartType::Anointments,
                    &self.parts_tab_type,
                    interaction_message(ItemEditorInteractionMessage::CurrentAnointmentsTabPressed),
                    Some(format!(
                        "({}/{})",
                        item.item_parts
                            .as_ref()
                            .map(|ip| ip.generic_parts().len())
                            .unwrap_or(0),
                        MAX_BL3_ITEM_ANOINTMENTS
                    )),
                ))
                .padding(1)
                .width(Length::FillPortion(2)),
            )
            .align_items(Alignment::Center);

        let mut current_parts_content = Column::new().push(Container::new(title_row));

        let parts;

        match self.parts_tab_type {
            CurrentPartType::Parts => {
                if reorder_parts {
                    parts = Self::regular_parts(item, all_parts_list);
                } else {
                    parts = Self::categorized_parts(item, all_parts_list);
                }

                self.parts = parts.clone();

                if !self.parts.is_empty() {
                    let mut reorder_parts_row =
                        Row::new().spacing(15).align_items(Alignment::Center);

                    let reorder_parts_tooltip =
                        "Reorder the parts of this item. The order that is shown when this checkbox is active will be the order that they get loaded in game. They will be loaded from top to bottom.\nEven if this checkbox is not active and you are viewing the categorized parts, they will still be loaded in the order that is show when this checkbox is active.";

                    let reorder_parts_checkbox = Tooltip::new(
                        Checkbox::new(self.reorder_parts, "Reorder Parts", move |c| {
                            interaction_message(
                                ItemEditorInteractionMessage::ReorderCurrentPartsSelected(c),
                            )
                        })
                        .size(17)
                        .font(JETBRAINS_MONO_BOLD)
                        .text_color(Color::from_rgb8(220, 220, 220))
                        .text_size(17)
                        .width(Length::Fill)
                        .style(Bl3UiStyle)
                        .into_element(),
                        reorder_parts_tooltip,
                        tooltip::Position::Top,
                    )
                    .gap(10)
                    .padding(10)
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .style(Bl3UiTooltipStyle);

                    reorder_parts_row = reorder_parts_row.push(reorder_parts_checkbox);

                    // Buttons to move selected part
                    if reorder_parts {
                        let reorder_move_up_button = Button::new(
                            &mut self.reorder_parts_move_up_button_state,
                            Text::new("Up")
                                .font(JETBRAINS_MONO_BOLD)
                                .size(17)
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .on_press(interaction_message(
                            ItemEditorInteractionMessage::ReorderCurrentPartsMoveUpPressed,
                        ))
                        .padding(5)
                        .width(Length::Units(70))
                        .style(Bl3UiStyle)
                        .into_element();

                        let reorder_move_down_button = Button::new(
                            &mut self.reorder_parts_move_down_button_state,
                            Text::new("Down")
                                .font(JETBRAINS_MONO_BOLD)
                                .size(17)
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .on_press(interaction_message(
                            ItemEditorInteractionMessage::ReorderCurrentPartsMoveDownPressed,
                        ))
                        .padding(5)
                        .width(Length::Units(70))
                        .style(Bl3UiStyle)
                        .into_element();

                        let reorder_move_top_button = Button::new(
                            &mut self.reorder_parts_move_top_button_state,
                            Text::new("Top")
                                .font(JETBRAINS_MONO_BOLD)
                                .size(17)
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .on_press(interaction_message(
                            ItemEditorInteractionMessage::ReorderCurrentPartsMoveTopPressed,
                        ))
                        .padding(5)
                        .width(Length::Units(70))
                        .style(Bl3UiStyle)
                        .into_element();

                        let reorder_move_bottom_button = Button::new(
                            &mut self.reorder_parts_move_bottom_button_state,
                            Text::new("Bottom")
                                .font(JETBRAINS_MONO_BOLD)
                                .size(17)
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .on_press(interaction_message(
                            ItemEditorInteractionMessage::ReorderCurrentPartsMoveBottomPressed,
                        ))
                        .padding(5)
                        .width(Length::Units(70))
                        .style(Bl3UiStyle)
                        .into_element();

                        reorder_parts_row = reorder_parts_row
                            .push(reorder_move_up_button)
                            .push(reorder_move_down_button)
                            .push(reorder_move_top_button)
                            .push(reorder_move_bottom_button);
                    }

                    current_parts_content = current_parts_content.push(
                        Container::new(
                            Container::new(reorder_parts_row)
                                .padding(15)
                                .width(Length::Fill)
                                .style(Bl3UiStyleNoBorder),
                        )
                        .padding(1),
                    );
                }
            }
            CurrentPartType::Anointments => {
                parts = Self::regular_anointments(item, anointments_list);

                self.parts = parts.clone();
            }
        }

        if !self.parts.is_empty() {
            let filtered_parts: Vec<&CurrentItemEditorPart> =
                filter_parts(&self.search_input, &parts);

            let amount: usize = parts.iter().map(|cat_p| cat_p.parts.len()).sum();

            let search_placeholder = match self.parts_tab_type {
                CurrentPartType::Parts => format!("Search {} Current Parts...", amount),
                CurrentPartType::Anointments => {
                    format!("Search {} Current Anointments...", amount)
                }
            };

            let search_input = TextInputLimited::new(
                &mut self.search_input_state,
                &search_placeholder,
                &self.search_input,
                500,
                move |s| {
                    interaction_message(
                        ItemEditorInteractionMessage::CurrentPartsSearchInputChanged(s),
                    )
                },
            )
            .0
            .font(JETBRAINS_MONO)
            .padding(10)
            .size(17)
            .style(Bl3UiStyle)
            .into_element();

            current_parts_content = current_parts_content.push(search_input);

            if !filtered_parts.is_empty() {
                let current_parts_list =
                    self.parts
                        .iter_mut()
                        .fold(Column::new(), |mut curr, cat_parts| {
                            if !reorder_parts
                                && cat_parts
                                    .parts
                                    .par_iter()
                                    .any(|cat_p| filtered_parts.contains(&cat_p))
                            {
                                curr = curr.push(
                                    Container::new(
                                        Text::new(&cat_parts.category)
                                            .font(JETBRAINS_MONO_BOLD)
                                            .size(17)
                                            .color(Color::from_rgb8(242, 203, 5)),
                                    )
                                    .width(Length::Fill)
                                    .style(Bl3UiStyleNoBorder)
                                    .padding(10),
                                );
                            }

                            for (part_index, p) in cat_parts.parts.iter_mut().enumerate() {
                                if filtered_parts.contains(&&*p) {
                                    let is_active =
                                        selected_current_part_index.part_index == part_index;

                                    curr = curr.push(p.view(
                                        reorder_parts,
                                        is_active,
                                        interaction_message,
                                    ))
                                }
                            }

                            curr
                        });

                current_parts_content = current_parts_content.push(
                    Container::new(
                        Scrollable::new(&mut self.scrollable_state)
                            .push(current_parts_list)
                            .height(Length::Fill)
                            .width(Length::Fill),
                    )
                    .padding(1),
                );
            } else {
                current_parts_content =
                    current_parts_content.push(no_parts_message(NO_SEARCH_RESULTS_FOUND_MESSAGE));
            }
        } else {
            let msg = match self.parts_tab_type {
                CurrentPartType::Parts => "This item has no parts.",
                CurrentPartType::Anointments => "This item has no anointments.",
            };

            current_parts_content = current_parts_content.push(no_parts_message(msg));
        }

        Container::new(current_parts_content)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .style(Bl3UiStyle)
    }

    pub fn regular_parts(
        item: &Bl3Item,
        all_parts_list: Option<&Vec<ResourceCategorizedParts>>,
    ) -> Vec<CurrentCategorizedPart> {
        let mut parts: Vec<Bl3PartWithInfo> = Vec::new();

        if let Some(all_parts_list) = all_parts_list {
            if let Some(item_parts) = &item.item_parts {
                item_parts.parts().iter().for_each(|p| {
                    let resource_part: Option<&ResourcePart> =
                        //Find extra info about the part
                        all_parts_list.iter().find_map(|cat_resource| {
                            cat_resource.parts.par_iter().find_first(|cat_part| {
                                part_contains(
                                    p.short_ident.as_ref(),
                                    &p.ident,
                                    &cat_part.name,
                                )
                            })
                        });

                    parts.push(Bl3PartWithInfo {
                        part: p.to_owned(),
                        info: resource_part
                            .map(|rp| rp.info.to_owned())
                            .unwrap_or_default(),
                    });
                });
            }
        }

        //Here we are just returning a single category with all of our parts above
        vec![CurrentCategorizedPart::new(
            0,
            "".to_owned(),
            CurrentPartType::Parts,
            parts,
        )]
    }

    pub fn categorized_parts(
        item: &Bl3Item,
        all_parts_list: Option<&Vec<ResourceCategorizedParts>>,
    ) -> Vec<CurrentCategorizedPart> {
        let mut categorized_parts: BTreeMap<String, Vec<Bl3PartWithInfo>> = BTreeMap::new();

        if let Some(all_parts_list) = all_parts_list {
            if let Some(item_parts) = &item.item_parts {
                item_parts.parts().iter().for_each(|p| {
                    let resource_part: Option<(&String, &ResourcePart)> =
                        //Find extra info about the part
                        all_parts_list.par_iter().find_map_any(|cat_resource| {
                            let part = cat_resource.parts.par_iter().find_first(|cat_part| {
                                part_contains(
                                    p.short_ident.as_ref(),
                                    &p.ident,
                                    &cat_part.name,
                                )
                            });

                            part.map(|part| (&cat_resource.category, part))
                        });

                    if let Some((category, resource_part)) = resource_part {
                        let curr_cat_parts = categorized_parts
                            .entry(category.to_owned())
                            .or_insert_with(Vec::new);

                        curr_cat_parts.push(Bl3PartWithInfo {
                            part: p.to_owned(),
                            info: resource_part.info.to_owned(),
                        });
                    } else {
                        let curr_cat_parts = categorized_parts
                            .entry("Unknown Parts".to_owned())
                            .or_insert_with(Vec::new);

                        curr_cat_parts.push(Bl3PartWithInfo {
                            part: p.to_owned(),
                            info: ResourcePartInfo::default(),
                        });
                    }
                });
            }
        } else if let Some(item_parts) = &item.item_parts {
            item_parts.parts().iter().for_each(|p| {
                let curr_cat_parts = categorized_parts
                    .entry("Unknown Parts".to_owned())
                    .or_insert_with(Vec::new);

                curr_cat_parts.push(Bl3PartWithInfo {
                    part: p.to_owned(),
                    info: ResourcePartInfo::default(),
                });
            })
        }

        let inventory_categorized_parts =
            categorized_parts.into_iter().map(|(category, mut parts)| {
                parts.par_sort();
                ItemEditorCategorizedParts { category, parts }
            });

        inventory_categorized_parts
            .into_iter()
            .enumerate()
            .map(|(cat_id, cat_p)| {
                CurrentCategorizedPart::new(
                    cat_id,
                    cat_p.category,
                    CurrentPartType::Parts,
                    cat_p.parts,
                )
            })
            .collect()
    }

    pub fn regular_anointments(
        item: &Bl3Item,
        anointments_list: &[ResourceCategorizedParts],
    ) -> Vec<CurrentCategorizedPart> {
        let mut categorized_parts: BTreeMap<String, Vec<Bl3PartWithInfo>> = BTreeMap::new();

        if let Some(item_parts) = &item.item_parts {
            item_parts.generic_parts().iter().for_each(|p| {
                let curr_cat_parts = categorized_parts
                    .entry("Anointment".to_owned())
                    .or_insert_with(Vec::new);

                //Find extra info about the anointment
                let resource_part: Option<&ResourcePart> =
                    anointments_list.par_iter().find_map_any(|cat_resource| {
                        let part = cat_resource.parts.par_iter().find_first(|cat_part| {
                            part_contains(p.short_ident.as_ref(), &p.ident, &cat_part.name)
                        });

                        part
                    });

                curr_cat_parts.push(Bl3PartWithInfo {
                    part: p.to_owned(),
                    info: resource_part
                        .map(|rp| rp.info.to_owned())
                        .unwrap_or_default(),
                });
            })
        }

        let inventory_categorized_anointments =
            categorized_parts.into_iter().map(|(category, mut parts)| {
                parts.par_sort();
                ItemEditorCategorizedParts { category, parts }
            });

        inventory_categorized_anointments
            .into_iter()
            .enumerate()
            .map(|(cat_id, cat_p)| {
                CurrentCategorizedPart::new(
                    cat_id,
                    cat_p.category,
                    CurrentPartType::Anointments,
                    cat_p.parts,
                )
            })
            .collect()
    }
}

fn part_contains(short_ident: Option<&String>, ident: &str, cat_part_name: &str) -> bool {
    if let Some(short_ident) = short_ident {
        cat_part_name.eq_ignore_ascii_case(short_ident)
    } else {
        let name_with_stop = format!("{}.", cat_part_name.to_lowercase());

        ident.to_lowercase().contains(&name_with_stop)
    }
}

fn no_parts_message<'a>(message: &str) -> Container<'a, Bl3Message> {
    Container::new(
        Text::new(message)
            .font(JETBRAINS_MONO)
            .size(17)
            .color(Color::from_rgb8(220, 220, 220)),
    )
    .height(Length::Fill)
    .width(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .padding(1)
}
