use heck::TitleCase;
use iced::{button, container, Button, Color, Column, Container, Element, Length, Row, Text};

use bl3_save_edit_core::bl3_save::bl3_item::{Bl3Item, ItemRarity, ItemType};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::inventory::inventory_button_style::InventoryButtonStyle;
use crate::views::manage_save::inventory::inventory_item_editor::InventoryItemEditor;
use crate::views::manage_save::inventory::SaveInventoryInteractionMessage;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;

#[derive(Debug, Default)]
pub struct InventoryListItem {
    pub id: usize,
    pub item: Bl3Item,
    button_state: button::State,
    pub editor: InventoryItemEditor,
}

impl InventoryListItem {
    pub fn new(id: usize, item: Bl3Item) -> Self {
        InventoryListItem {
            id,
            item,
            ..Default::default()
        }
    }

    pub fn view(&mut self, is_active: bool) -> (Element<Message>, Option<Container<Message>>) {
        let balance_part = self.item.balance_part();

        let label = balance_part.name.clone().unwrap_or_else(|| {
            balance_part
                .short_ident
                .clone()
                .unwrap_or_else(|| balance_part.ident.clone())
        });

        let mut tags_row = Row::new()
            .push(
                Container::new(
                    Text::new(format!("Level {}", self.item.level()))
                        .font(JETBRAINS_MONO)
                        .size(15),
                )
                .padding(5)
                .style(ItemInfoStyle),
            )
            .width(Length::Fill)
            .spacing(10);

        if let Some(mut manufacturer_short) = self.item.manufacturer_part().short_ident.clone() {
            if manufacturer_short != "CoV" {
                manufacturer_short = manufacturer_short.to_title_case();
            }

            tags_row = tags_row.push(
                Container::new(Text::new(manufacturer_short).font(JETBRAINS_MONO).size(15))
                    .padding(5)
                    .style(ItemInfoStyle),
            )
        }

        if let Some(item_parts) = &self.item.item_parts {
            match &item_parts.item_type {
                ItemType::Weapon => {
                    if let Some(weapon_type) = &item_parts.weapon_type {
                        tags_row = tags_row.push(
                            Container::new(
                                Text::new(weapon_type.to_string())
                                    .font(JETBRAINS_MONO)
                                    .size(15),
                            )
                            .padding(5)
                            .style(ItemTypeStyle),
                        )
                    }
                }
                other => {
                    tags_row = tags_row.push(
                        Container::new(Text::new(other.to_string()).font(JETBRAINS_MONO).size(15))
                            .padding(5)
                            .style(ItemTypeStyle),
                    )
                }
            }

            tags_row = tags_row.push(
                Container::new(
                    Text::new(item_parts.rarity.to_string())
                        .font(JETBRAINS_MONO)
                        .size(15),
                )
                .padding(5)
                .style(ItemRarityStyle {
                    rarity: item_parts.rarity.clone(),
                }),
            );
        }

        let button_content = Column::new()
            .push(Text::new(&label).font(JETBRAINS_MONO).size(17))
            .push(tags_row)
            .spacing(10);

        let item_editor = if is_active {
            Some(self.editor.view(self.id, &self.item))
        } else {
            None
        };

        (
            Button::new(&mut self.button_state, Container::new(button_content))
                .on_press(InteractionMessage::ManageSaveInteraction(
                    ManageSaveInteractionMessage::Inventory(
                        SaveInventoryInteractionMessage::ItemPressed(self.id),
                    ),
                ))
                .padding(10)
                .width(Length::Fill)
                .style(InventoryButtonStyle { is_active })
                .into_element(),
            item_editor,
        )
    }
}

struct ItemInfoStyle;

impl container::StyleSheet for ItemInfoStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(224, 224, 224)),
            background: Some(Color::from_rgb8(38, 38, 38).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(46, 46, 46),
        }
    }
}

struct ItemTypeStyle;

impl container::StyleSheet for ItemTypeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(240, 149, 149)),
            background: Some(Color::from_rgb8(54, 29, 29).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(61, 36, 36),
        }
    }
}

struct ItemRarityStyle {
    rarity: ItemRarity,
}

impl container::StyleSheet for ItemRarityStyle {
    fn style(&self) -> container::Style {
        let (text_color, background, border_color) = match self.rarity {
            ItemRarity::Common => (
                Color::from_rgb8(242, 233, 218),
                Color::from_rgb8(54, 51, 48),
                Color::from_rgb8(61, 59, 55),
            ),
            ItemRarity::Uncommon => (
                Color::from_rgb8(172, 240, 149),
                Color::from_rgb8(35, 54, 29),
                Color::from_rgb8(42, 61, 36),
            ),
            ItemRarity::Rare => (
                Color::from_rgb8(149, 202, 240),
                Color::from_rgb8(29, 44, 54),
                Color::from_rgb8(36, 51, 61),
            ),
            ItemRarity::VeryRare => (
                Color::from_rgb8(208, 149, 240),
                Color::from_rgb8(45, 29, 54),
                Color::from_rgb8(52, 36, 61),
            ),
            ItemRarity::Legendary => (
                Color::from_rgb8(240, 213, 149),
                Color::from_rgb8(54, 47, 29),
                Color::from_rgb8(61, 54, 36),
            ),
        };

        container::Style {
            text_color: Some(text_color),
            background: Some(background.into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color,
        }
    }
}
