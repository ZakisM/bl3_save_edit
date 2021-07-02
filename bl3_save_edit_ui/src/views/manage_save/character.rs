use std::ops::Deref;

use iced::{
    container, pick_list, text_input, Align, Color, Column, Container, HorizontalAlignment, Length,
    PickList, Row, Text, TextInput,
};

use bl3_save_edit_core::bl3_save::player_class::PlayerClass;

use crate::bl3_ui::Message;
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::ManageSaveMessage;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_margin::TextMargin;

#[derive(Debug, Default)]
pub struct CharacterState {
    pub name_input: String,
    pub name_input_state: text_input::State,
    pub player_class_selector: pick_list::State<PlayerClass>,
    pub player_class_selected_class: PlayerClass,
}

#[derive(Debug, Clone)]
pub enum CharacterMessage {
    CharacterNameInputChanged(String),
    PlayerClassSelected(PlayerClass),
}

pub fn view(character_state: &mut CharacterState) -> Container<Message> {
    let character_name = Container::new(
        Row::new()
            .push(
                TextMargin::new("Name", 2)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(242, 203, 5))
                    .width(Length::Units(90)),
            )
            .push(
                TextInput::new(
                    &mut character_state.name_input_state,
                    "Amara",
                    &character_state.name_input,
                    |s| {
                        Message::ManageSave(ManageSaveMessage::Character(
                            CharacterMessage::CharacterNameInputChanged(s),
                        ))
                    },
                )
                .font(JETBRAINS_MONO)
                .padding(10)
                .size(17)
                .style(Bl3UiStyle),
            )
            .align_items(Align::Center),
    )
    .width(Length::FillPortion(3))
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let save_slot = Container::new(
        Row::new()
            .push(
                TextMargin::new("Class", 2)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(242, 203, 5))
                    .width(Length::Units(90)),
            )
            .push(
                PickList::new(
                    &mut character_state.player_class_selector,
                    &PlayerClass::ALL[..],
                    Some(character_state.player_class_selected_class),
                    |s| {
                        Message::ManageSave(ManageSaveMessage::Character(
                            CharacterMessage::PlayerClassSelected(s),
                        ))
                    },
                )
                .font(JETBRAINS_MONO)
                .text_size(17)
                .width(Length::Fill)
                .padding(10)
                .style(Bl3UiStyle),
            )
            .align_items(Align::Center),
    )
    .width(Length::FillPortion(1))
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let name_class_row = Row::new().push(character_name).push(save_slot).spacing(20);

    let all_contents = Column::new().push(name_class_row).spacing(20);

    Container::new(all_contents).padding(30)
}
