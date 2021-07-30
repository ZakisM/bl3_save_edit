use iced::{text_input, Align, Column, Container, Length, Row};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

#[derive(Debug, Default)]
pub struct CurrencyState {
    pub money_input: i32,
    pub money_input_state: text_input::State,
    pub eridium_input: i32,
    pub eridium_input_state: text_input::State,
}

#[derive(Debug)]
pub enum CurrencyMessage {}

#[derive(Debug, Clone)]
pub enum CurrencyInteractionMessage {
    MoneyInputChanged(i32),
    EridiumInputChanged(i32),
}

pub fn view(general_state: &mut CurrencyState) -> Container<Message> {
    let money = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "Money",
                    Length::Units(75),
                    NumberInput::new(
                        &mut general_state.money_input_state,
                        general_state.money_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageSaveInteraction(
                                ManageSaveInteractionMessage::Currency(
                                    CurrencyInteractionMessage::MoneyInputChanged(v),
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
                .width(Length::FillPortion(9))
                .align_items(Align::Center),
            )
            .align_items(Align::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let eridium = Container::new(
        LabelledElement::create(
            "Eridium",
            Length::Units(75),
            NumberInput::new(
                &mut general_state.eridium_input_state,
                general_state.eridium_input,
                0,
                None,
                |v| {
                    InteractionMessage::ManageSaveInteraction(
                        ManageSaveInteractionMessage::Currency(
                            CurrencyInteractionMessage::EridiumInputChanged(v),
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

    let all_contents = Column::new().push(money).push(eridium).spacing(20);

    Container::new(all_contents).padding(30)
}
