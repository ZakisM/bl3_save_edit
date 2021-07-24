use iced::{
    button, pick_list, text_input, tooltip, Align, Button, Column, Container, Length, PickList,
    Row, Text, TextInput, Tooltip,
};

use bl3_save_edit_core::parser::HeaderType;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::interaction::InteractionExt;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

#[derive(Debug, Default)]
pub struct GeneralState {
    pub guid_input: String,
    pub guid_input_state: text_input::State,
    pub slot_input: u32,
    pub slot_input_state: text_input::State,
    pub generate_guid_button_state: button::State,
    pub save_type_selector: pick_list::State<HeaderType>,
    pub save_type_selected: HeaderType,
}

#[derive(Debug, Clone)]
pub enum GeneralMessage {
    GenerateRandomGuidCompleted(String),
}

#[derive(Debug, Clone)]
pub enum GeneralInteractionMessage {
    GuidInputChanged(String),
    SlotInputChanged(u32),
    GenerateGuidPressed,
    SaveTypeSelected(HeaderType),
}

pub fn view(general_state: &mut GeneralState) -> Container<Message> {
    let save_guid = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "Save GUID",
                    Length::Units(90),
                    TextInput::new(
                        &mut general_state.guid_input_state,
                        "00000000000000000000000000000000",
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
                .width(Length::FillPortion(9))
                .align_items(Align::Center),
            )
            .push(
                Button::new(
                    &mut general_state.generate_guid_button_state,
                    Text::new("Random").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::ManageSaveInteraction(
                    ManageSaveInteractionMessage::General(
                        GeneralInteractionMessage::GenerateGuidPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Align::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let save_slot = Container::new(
        LabelledElement::create(
            "Save Slot",
            Length::Units(90),
            Tooltip::new(
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
                "Slot must be 1 or greater",
                tooltip::Position::Top,
            )
            .gap(10)
            .padding(10)
            .font(JETBRAINS_MONO)
            .size(17)
            .style(Bl3UiTooltipStyle),
        )
        .spacing(15)
        .align_items(Align::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let save_type = Container::new(
        LabelledElement::create(
            "Save Type",
            Length::Units(90),
            PickList::new(
                &mut general_state.save_type_selector,
                &HeaderType::SAVE_TYPES[..],
                Some(general_state.save_type_selected),
                |h| {
                    InteractionMessage::ManageSaveInteraction(
                        ManageSaveInteractionMessage::General(
                            GeneralInteractionMessage::SaveTypeSelected(h),
                        ),
                    )
                },
            )
            .font(JETBRAINS_MONO)
            .text_size(17)
            .width(Length::Fill)
            .padding(10)
            .style(Bl3UiStyle)
            .into_element(),
        )
        .spacing(15)
        .align_items(Align::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let all_contents = Column::new()
        .push(save_guid)
        .push(save_slot)
        .push(save_type)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
