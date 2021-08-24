use iced::{pick_list, text_input, Align, Column, Container, Length, PickList, Row};

use bl3_save_edit_core::bl3_profile::science_levels::BorderlandsScienceLevel;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_profile::profile::sdu::SduUnlocker;
use crate::views::manage_profile::profile::skin_unlocker::SkinUnlocker;
use crate::views::manage_profile::ManageProfileInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

pub mod sdu;
pub mod skin_unlocker;

#[derive(Debug, Default)]
pub struct ProfileState {
    pub guardian_rank_input: i32,
    pub guardian_rank_input_state: text_input::State,
    pub guardian_rank_tokens_input: i32,
    pub guardian_rank_tokens_input_state: text_input::State,
    pub science_level_selector: pick_list::State<BorderlandsScienceLevel>,
    pub science_level_selected: BorderlandsScienceLevel,
    pub science_tokens_input: i32,
    pub science_tokens_input_state: text_input::State,
    pub skin_unlocker: SkinUnlocker,
    pub sdu_unlocker: SduUnlocker,
}

#[derive(Debug, Clone)]
pub enum ProfileProfileInteractionMessage {
    GuardianRankInputChanged(i32),
    GuardianRankTokensInputChanged(i32),
    ScienceLevelSelected(BorderlandsScienceLevel),
    ScienceTokensInputChanged(i32),
    SkinMessage(ProfileSkinUnlockedMessage),
    SduMessage(ProfileSduInputChangedMessage),
    MaxSduSlotsPressed,
}

#[derive(Debug, Clone)]
pub enum ProfileSkinUnlockedMessage {
    CharacterSkins(bool),
    CharacterHeads(bool),
    EchoThemes(bool),
    Emotes(bool),
    RoomDecorations(bool),
    WeaponSkins(bool),
    WeaponTrinkets(bool),
}

#[derive(Debug, Clone)]
pub enum ProfileSduInputChangedMessage {
    Bank(i32),
    LostLoot(i32),
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
    .width(Length::FillPortion(2))
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let borderlands_science_tokens = Container::new(
        LabelledElement::create(
            "Borderlands Science Tokens",
            Length::Units(225),
            NumberInput::new(
                &mut profile_state.science_tokens_input_state,
                profile_state.science_tokens_input,
                0,
                None,
                |v| {
                    InteractionMessage::ManageProfileInteraction(
                        ManageProfileInteractionMessage::Profile(
                            ProfileProfileInteractionMessage::ScienceTokensInputChanged(v),
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
    .width(Length::FillPortion(2))
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let borderlands_science_row = Row::new()
        .push(borderlands_science_level)
        .push(borderlands_science_tokens)
        .spacing(20);

    let skin_unlocker = profile_state
        .skin_unlocker
        .view()
        .width(Length::FillPortion(3));

    let sdu_unlocker = profile_state
        .sdu_unlocker
        .view()
        .width(Length::FillPortion(2));

    let skin_sdu_row = Row::new()
        .push(skin_unlocker)
        .push(sdu_unlocker)
        .spacing(20);

    let all_contents = Column::new()
        .push(guardian_rank)
        .push(guardian_rank_tokens)
        .push(borderlands_science_row)
        .push(skin_sdu_row)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
