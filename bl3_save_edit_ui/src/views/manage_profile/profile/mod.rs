use iced::{pick_list, text_input, Alignment, Column, Container, Length, PickList, Row};

use bl3_save_edit_core::bl3_profile::science_levels::BorderlandsScienceLevel;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_profile::profile::guardian_rewards::GuardianRewardUnlocker;
use crate::views::manage_profile::profile::sdu::SduUnlocker;
use crate::views::manage_profile::profile::skin_unlocker::SkinUnlocker;
use crate::views::manage_profile::ManageProfileInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

pub mod guardian_rewards;
pub mod sdu;
pub mod skin_unlocker;

#[derive(Debug, Default)]
pub struct ProfileState {
    pub guardian_rank_tokens_input: i32,
    pub guardian_rank_tokens_input_state: text_input::State,
    pub science_level_selector: pick_list::State<BorderlandsScienceLevel>,
    pub science_level_selected: BorderlandsScienceLevel,
    pub science_tokens_input: i32,
    pub science_tokens_input_state: text_input::State,
    pub skin_unlocker: SkinUnlocker,
    pub sdu_unlocker: SduUnlocker,
    pub guardian_reward_unlocker: GuardianRewardUnlocker,
}

#[derive(Debug, Clone)]
pub enum ProfileInteractionMessage {
    GuardianRankTokens(i32),
    ScienceLevelSelected(BorderlandsScienceLevel),
    ScienceTokens(i32),
    SkinMessage(SkinUnlockedMessage),
    SduMessage(SduMessage),
    MaxSduSlotsPressed,
    GuardianRewardMessage(GuardianRewardMessage),
    MaxGuardianRewardsPressed,
}

#[derive(Debug, Clone)]
pub enum SkinUnlockedMessage {
    CharacterSkins(bool),
    CharacterHeads(bool),
    EchoThemes(bool),
    Emotes(bool),
    RoomDecorations(bool),
    WeaponSkins(bool),
    WeaponTrinkets(bool),
}

#[derive(Debug, Clone)]
pub enum SduMessage {
    Bank(i32),
    LostLoot(i32),
}

#[derive(Debug, Clone)]
pub enum GuardianRewardMessage {
    Accuracy(i32),
    ActionSkillCooldown(i32),
    CriticalDamage(i32),
    ElementalDamage(i32),
    FFYLDuration(i32),
    FFYLMovementSpeed(i32),
    GrenadeDamage(i32),
    GunDamage(i32),
    GunFireRate(i32),
    MaxHealth(i32),
    MeleeDamage(i32),
    RarityRate(i32),
    RecoilReduction(i32),
    ReloadSpeed(i32),
    ShieldCapacity(i32),
    ShieldRechargeDelay(i32),
    ShieldRechargeRate(i32),
    VehicleDamage(i32),
}

pub fn view(profile_state: &mut ProfileState) -> Container<Bl3Message> {
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
                            ProfileInteractionMessage::GuardianRankTokens(v),
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
        .align_items(Alignment::Center),
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
                            ProfileInteractionMessage::ScienceLevelSelected(h),
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
        .align_items(Alignment::Center),
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
                            ProfileInteractionMessage::ScienceTokens(v),
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
        .align_items(Alignment::Center),
    )
    .width(Length::FillPortion(2))
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let borderlands_science_row = Row::new()
        .push(borderlands_science_level)
        .push(borderlands_science_tokens)
        .spacing(20);

    let guardian_reward_unlocker = profile_state
        .guardian_reward_unlocker
        .view()
        .width(Length::Fill);

    let main_column = Container::new(
        Column::new()
            .push(guardian_rank_tokens)
            .push(borderlands_science_row)
            .push(guardian_reward_unlocker)
            .spacing(20),
    )
    .height(Length::Units(560))
    .width(Length::Fill);

    let skin_unlocker = profile_state.skin_unlocker.view();

    let sdu_unlocker = profile_state.sdu_unlocker.view();

    let skin_unlocker_sdu_unlocker_column = Container::new(
        Column::new()
            .push(skin_unlocker)
            .push(sdu_unlocker)
            .spacing(20),
    )
    .height(Length::Units(560))
    .width(Length::Units(360));

    let all_contents = Row::new()
        .push(main_column)
        .push(skin_unlocker_sdu_unlocker_column)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
