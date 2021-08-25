use iced::{text_input, Align, Column, Container, Length, Row};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_profile::ManageProfileInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

#[derive(Debug, Default)]
pub struct KeysState {
    pub golden_keys_input: i32,
    pub golden_keys_input_state: text_input::State,
    pub diamond_keys_input: i32,
    pub diamond_keys_input_state: text_input::State,
    pub vault_card_1_keys_input: i32,
    pub vault_card_1_keys_input_state: text_input::State,
    pub vault_card_1_chests_input: i32,
    pub vault_card_1_chests_input_state: text_input::State,
}

#[derive(Debug, Clone)]
pub enum ProfileKeysInteractionMessage {
    GoldenKeys(i32),
    DiamondKeys(i32),
    VaultCard1Keys(i32),
    VaultCard1Chests(i32),
}

pub fn view(keys_state: &mut KeysState) -> Container<Message> {
    let golden_keys = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "Golden Keys",
                    Length::Units(170),
                    NumberInput::new(
                        &mut keys_state.golden_keys_input_state,
                        keys_state.golden_keys_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Keys(
                                    ProfileKeysInteractionMessage::GoldenKeys(v),
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

    let diamond_keys = Container::new(
        LabelledElement::create(
            "Diamond Keys",
            Length::Units(170),
            NumberInput::new(
                &mut keys_state.diamond_keys_input_state,
                keys_state.diamond_keys_input,
                0,
                None,
                |v| {
                    InteractionMessage::ManageProfileInteraction(
                        ManageProfileInteractionMessage::Keys(
                            ProfileKeysInteractionMessage::DiamondKeys(v),
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

    let vault_card_1_keys = Container::new(
        LabelledElement::create(
            "Vault Card 1 Keys",
            Length::Units(170),
            NumberInput::new(
                &mut keys_state.vault_card_1_keys_input_state,
                keys_state.vault_card_1_keys_input,
                0,
                None,
                |v| {
                    InteractionMessage::ManageProfileInteraction(
                        ManageProfileInteractionMessage::Keys(
                            ProfileKeysInteractionMessage::VaultCard1Keys(v),
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

    let vault_card_1_chests = Container::new(
        LabelledElement::create(
            "Vault Card 1 Chests",
            Length::Units(170),
            NumberInput::new(
                &mut keys_state.vault_card_1_chests_input_state,
                keys_state.vault_card_1_chests_input,
                0,
                None,
                |v| {
                    InteractionMessage::ManageProfileInteraction(
                        ManageProfileInteractionMessage::Keys(
                            ProfileKeysInteractionMessage::VaultCard1Chests(v),
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

    let all_contents = Column::new()
        .push(golden_keys)
        .push(diamond_keys)
        .push(vault_card_1_keys)
        .push(vault_card_1_chests)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
