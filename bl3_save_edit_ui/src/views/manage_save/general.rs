use iced::{text_input, Align, Color, Column, Container, Length, Row, TextInput};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::interaction::InteractionExt;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_margin::TextMargin;

#[derive(Debug, Default)]
pub struct GeneralState {
    pub guid_input: String,
    pub guid_input_state: text_input::State,
    pub slot_input: u32,
    pub slot_input_state: text_input::State,
}

#[derive(Debug, Clone)]
pub enum GeneralInteractionMessage {
    GuidInputChanged(String),
    SlotInputChanged(u32),
}

pub fn view(general_state: &mut GeneralState) -> Container<Message> {
    let save_guid = Container::new(
        Row::new()
            .push(
                TextMargin::new("Save GUID", 2)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(242, 203, 5))
                    .width(Length::Units(90)),
            )
            .push(
                TextInput::new(
                    &mut general_state.guid_input_state,
                    "00000000-0000-0000-0000-000000000000",
                    &general_state.guid_input,
                    |s| {
                        InteractionMessage::ManageSaveInteraction(
                            ManageSaveInteractionMessage::General(
                                GeneralInteractionMessage::GuidInputChanged(s),
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
            .align_items(Align::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let save_slot = Container::new(
        Row::new()
            .push(
                TextMargin::new("Save Slot", 2)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(242, 203, 5))
                    .width(Length::Units(90)),
            )
            .push(
                NumberInput::new(
                    &mut general_state.slot_input_state,
                    general_state.slot_input,
                    1,
                    None,
                    |v| {
                        InteractionMessage::ManageSaveInteraction(
                            ManageSaveInteractionMessage::General(
                                GeneralInteractionMessage::SlotInputChanged(v),
                            ),
                        )
                    },
                )
                .0
                .font(JETBRAINS_MONO)
                .padding(10)
                .size(17)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .spacing(15)
            .align_items(Align::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let all_contents = Column::new().push(save_guid).push(save_slot).spacing(20);

    Container::new(all_contents).padding(30)
}
