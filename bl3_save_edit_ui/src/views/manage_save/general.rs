use iced::{container, text_input, Align, Color, Column, Container, Length, Row, Text, TextInput};

use crate::bl3_ui::Message;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::ManageSaveMessage;

#[derive(Debug, Default)]
pub struct GeneralState {
    pub guid_input: String,
    pub guid_input_state: text_input::State,
    pub slot_input: usize,
    pub slot_state: text_input::State,
}

#[derive(Debug, Clone)]
pub enum GeneralMessage {
    GuidInputChanged(String),
    SlotInputChanged(usize),
}

pub struct GeneralStyle;

impl container::StyleSheet for GeneralStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(25, 25, 25).into(),
            // border_width: 1.0,
            // border_radius: 2.5,
            // border_color: Color::from_rgb8(128, 106, 0),
            ..container::Style::default()
        }
    }
}

impl text_input::StyleSheet for GeneralStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Color::from_rgb8(25, 25, 25).into(),
            // border_width: 1.0,
            // border_radius: 2.5,
            // border_color: Color::from_rgb8(128, 106, 0),
            ..text_input::Style::default()
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Color::from_rgb8(25, 25, 25).into(),
            // border_width: 1.0,
            // border_radius: 2.5,
            // border_color: Color::from_rgb8(161, 134, 2),
            ..text_input::Style::default()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb8(179, 156, 39)
    }

    fn value_color(&self) -> Color {
        Color::from_rgb8(242, 203, 5)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgba8(179, 156, 39, 0.1)
    }
}

pub fn view(general_state: &mut GeneralState) -> Container<Message> {
    let save_guid = Container::new(
        Row::new()
            .push(
                Container::new(
                    Text::new("Save GUID:")
                        .font(JETBRAINS_MONO_BOLD)
                        .size(17)
                        .color(Color::from_rgb8(242, 203, 5)),
                )
                .align_x(Align::End)
                .width(Length::Units(85)),
            )
            .push(
                TextInput::new(
                    &mut general_state.guid_input_state,
                    "00000000-0000-0000-0000-000000000000",
                    &general_state.guid_input,
                    |s| {
                        Message::ManageSave(ManageSaveMessage::General(
                            GeneralMessage::GuidInputChanged(s),
                        ))
                    },
                )
                .font(JETBRAINS_MONO)
                .padding(10)
                .size(17)
                .style(GeneralStyle),
            )
            .spacing(15)
            .align_items(Align::Center)
            .width(Length::Fill)
            .height(Length::Units(40)),
    )
    .style(GeneralStyle);

    let save_slot = Container::new(
        Row::new()
            .push(
                Container::new(
                    Text::new("Save Slot:")
                        .font(JETBRAINS_MONO_BOLD)
                        .size(17)
                        .color(Color::from_rgb8(242, 203, 5)),
                )
                .align_x(Align::End)
                .width(Length::Units(85)),
            )
            .push(
                TextInput::new(
                    &mut general_state.slot_state,
                    "25",
                    &general_state.slot_input.to_string().replace("0", ""),
                    |s| {
                        if s.is_empty() {
                            Message::ManageSave(ManageSaveMessage::General(
                                GeneralMessage::SlotInputChanged(0),
                            ))
                        } else if let Ok(s) = s.parse::<usize>() {
                            Message::ManageSave(ManageSaveMessage::General(
                                GeneralMessage::SlotInputChanged(s),
                            ))
                        } else {
                            Message::Ignore
                        }
                    },
                )
                .font(JETBRAINS_MONO)
                .padding(10)
                .size(17)
                .style(GeneralStyle),
            )
            .spacing(15)
            .align_items(Align::Center)
            .width(Length::Fill)
            .height(Length::Units(40)),
    )
    .style(GeneralStyle);

    let all_contents = Column::new().push(save_guid).push(save_slot).spacing(20);

    Container::new(all_contents).padding(30)
}
