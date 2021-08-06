use iced::{button, container, Align, Button, Color, Container, Element, Length, Row, Text};

use bl3_save_edit_core::bl3_save::bl3_item::{Bl3Item, ItemRarity};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::inventory::inventory_button_style::InventoryButtonStyle;
use crate::views::manage_save::inventory::InventoryInteractionMessage;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;

#[derive(Debug)]
pub struct InventoryItem {
    id: usize,
    pub item: Bl3Item,
    label: String,
    button_state: button::State,
}

impl InventoryItem {
    pub fn new(id: usize, item: Bl3Item) -> Self {
        let balance_part = &item.balance_part;

        let label = balance_part.name.clone().unwrap_or_else(|| {
            balance_part
                .short_ident
                .clone()
                .unwrap_or_else(|| balance_part.ident.clone())
        });

        InventoryItem {
            id,
            label,
            item,
            button_state: button::State::new(),
        }
    }

    pub fn view(&mut self, is_active: bool) -> Element<Message> {
        let button_content = Row::new()
            .push(Text::new(&self.label).font(JETBRAINS_MONO).size(17))
            .push(
                Container::new(
                    Text::new(format!("Level {}", self.item.level))
                        .font(JETBRAINS_MONO_BOLD)
                        .size(15),
                )
                .padding(4)
                .style(ItemLevelStyle),
            )
            .push(
                Container::new(
                    Text::new(self.item.rarity.to_string())
                        .font(JETBRAINS_MONO_BOLD)
                        .size(15),
                )
                .padding(4)
                .style(ItemRarityStyle {
                    rarity: self.item.rarity.clone(),
                }),
            );

        Button::new(
            &mut self.button_state,
            button_content.align_items(Align::Center).spacing(10),
        )
        .on_press(InteractionMessage::ManageSaveInteraction(
            ManageSaveInteractionMessage::Inventory(InventoryInteractionMessage::ItemPressed(
                self.id,
            )),
        ))
        .padding(10)
        .width(Length::Fill)
        .style(InventoryButtonStyle { is_active })
        .into_element()
    }
}

struct ItemLevelStyle;

impl container::StyleSheet for ItemLevelStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(0, 0, 0)),
            background: Some(Color::from_rgb8(220, 220, 220).into()),
            border_radius: 3.0,
            border_width: 0.0,
            border_color: Default::default(),
        }
    }
}

struct ItemRarityStyle {
    rarity: ItemRarity,
}

impl container::StyleSheet for ItemRarityStyle {
    fn style(&self) -> container::Style {
        let background = match self.rarity {
            ItemRarity::Common => Color::from_rgb8(209, 205, 199),
            ItemRarity::Uncommon => Color::from_rgb8(123, 244, 81),
            ItemRarity::Rare => Color::from_rgb8(47, 169, 255),
            ItemRarity::VeryRare => Color::from_rgb8(203, 109, 255),
            ItemRarity::Legendary => Color::from_rgb8(255, 198, 65),
        };

        container::Style {
            text_color: Some(Color::from_rgb8(0, 0, 0)),
            background: Some(background.into()),
            border_radius: 3.0,
            border_width: 0.0,
            border_color: Default::default(),
        }
    }
}
