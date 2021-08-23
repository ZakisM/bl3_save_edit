use iced::{pick_list, text_input, Align, Column, Container, Length, PickList};

use bl3_save_edit_core::bl3_profile::science_levels::BorderlandsScienceLevel;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_profile::ManageProfileInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

#[derive(Debug, Default)]
pub struct ProfileState {
    pub guardian_rank_input: i32,
    pub guardian_rank_input_state: text_input::State,
    pub guardian_rank_tokens_input: i32,
    pub guardian_rank_tokens_input_state: text_input::State,
    pub science_level_selector: pick_list::State<BorderlandsScienceLevel>,
    pub science_level_selected: BorderlandsScienceLevel,
}

#[derive(Debug, Clone)]
pub enum ProfileProfileInteractionMessage {
    GuardianRankInputChanged(i32),
    GuardianRankTokensInputChanged(i32),
    ScienceLevelSelected(BorderlandsScienceLevel),
}

pub fn view(profile_state: &mut ProfileState) -> Container<Message> {
    let guardian_rank = Container::new(
        LabelledElement::create(
            "Guardian Rank",
            Length::Units(215),
            NumberInput::new(
                &mut profile_state.guardian_rank_input_state,
                profile_state.guardian_rank_input,
                0,
                None,
                |v| {
                    InteractionMessage::ManageProfileInteraction(
                        ManageProfileInteractionMessage::Profile(
                            ProfileProfileInteractionMessage::GuardianRankInputChanged(v),
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

    let guardian_rank_tokens = Container::new(
        LabelledElement::create(
            "Guardian Rank Tokens",
            Length::Units(215),
            NumberInput::new(
                &mut profile_state.guardian_rank_tokens_input_state,
                profile_state.guardian_rank_tokens_input,
                0,
                None,
                |v| {
                    InteractionMessage::ManageProfileInteraction(
                        ManageProfileInteractionMessage::Profile(
                            ProfileProfileInteractionMessage::GuardianRankTokensInputChanged(v),
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

    let borderlands_science_level = Container::new(
        LabelledElement::create(
            "Borderlands Science Level",
            Length::Units(215),
            PickList::new(
                &mut profile_state.science_level_selector,
                &BorderlandsScienceLevel::ALL[..],
                Some(profile_state.science_level_selected),
                |h| {
                    InteractionMessage::ManageProfileInteraction(
                        ManageProfileInteractionMessage::Profile(
                            ProfileProfileInteractionMessage::ScienceLevelSelected(h),
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

    //TODO: Borderlands Science Tokens and maybe Solves?

    let all_contents = Column::new()
        .push(guardian_rank)
        .push(guardian_rank_tokens)
        .push(borderlands_science_level)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
