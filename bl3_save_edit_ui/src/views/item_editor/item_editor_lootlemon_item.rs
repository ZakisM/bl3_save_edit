use iced::{button, Button, Color, Container, Element, HorizontalAlignment, Length, Row, Text};

use bl3_save_edit_core::bl3_item::Bl3Item;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiStyleCustomNoBorder};
use crate::resources::fonts::JETBRAINS_MONO_BOLD;
use crate::views::item_editor::{list_item_contents, ItemEditorInteractionMessage};
use crate::views::InteractionExt;

#[derive(Debug, Default)]
pub struct ItemEditorLootlemonItem {
    pub id: usize,
    pub item: Bl3Item,
    pub link: String,
    pub import_button_state: button::State,
    pub open_lootlemon_button_state: button::State,
}

impl ItemEditorLootlemonItem {
    pub fn new(id: usize, link: String, item: Bl3Item) -> Self {
        ItemEditorLootlemonItem {
            id,
            item,
            link,
            ..Self::default()
        }
    }

    pub fn view<F>(&mut self, view_index: usize, interaction_message: F) -> Element<Bl3Message>
    where
        F: Fn(ItemEditorInteractionMessage) -> InteractionMessage + 'static + Copy,
    {
        let options_rows = Row::new()
            .push(
                Button::new(
                    &mut self.import_button_state,
                    Text::new("Import Item")
                        .font(JETBRAINS_MONO_BOLD)
                        .size(17)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .on_press(interaction_message(
                    ItemEditorInteractionMessage::ItemListLootlemonImportPressed(self.id),
                ))
                .padding(5)
                .width(Length::Units(160))
                .style(Bl3UiStyle),
            )
            .push(
                Button::new(
                    &mut self.open_lootlemon_button_state,
                    Text::new("Open on Lootlemon ➜")
                        .font(JETBRAINS_MONO_BOLD)
                        .size(17)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .on_press(interaction_message(
                    ItemEditorInteractionMessage::ItemListLootlemonOpenWebsitePressed(self.id),
                ))
                .padding(5)
                .width(Length::Units(160))
                .style(Bl3UiStyle),
            )
            .width(Length::Fill)
            .spacing(10);

        let item_content = list_item_contents::view(&self.item).push(options_rows);

        let mut view = Container::new(item_content).padding(10).width(Length::Fill);

        if view_index % 2 == 0 {
            view = view.style(Bl3UiStyleCustomNoBorder(Color::from_rgb8(25, 25, 25)));
        } else {
            view = view.style(Bl3UiStyleCustomNoBorder(Color::from_rgb8(28, 28, 28)));
        }

        view.into_element()
    }
}
