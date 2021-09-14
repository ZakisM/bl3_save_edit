use iced::{
    pick_list, text_input, tooltip, Align, Column, Container, Length, PickList, Row, Tooltip,
};

use bl3_save_edit_core::bl3_save::character_data::MAX_CHARACTER_LEVEL;
use bl3_save_edit_core::bl3_save::player_class::PlayerClass;
use bl3_save_edit_core::bl3_save::util::REQUIRED_XP_LIST;
use bl3_save_edit_core::game_data::GameDataKv;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::character::ammo::AmmoSetter;
use crate::views::manage_save::character::gear::GearUnlocker;
use crate::views::manage_save::character::sdu::SduUnlocker;
use crate::views::manage_save::character::skins::SkinSelectors;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_input_limited::TextInputLimited;

mod ammo;
mod gear;
mod sdu;
mod skins;

#[derive(Debug, Default)]
pub struct CharacterState {
    pub name_input: String,
    pub name_input_state: text_input::State,
    pub player_class_selector: pick_list::State<PlayerClass>,
    pub player_class_selected_class: PlayerClass,
    pub level_input: i32,
    pub xp_level_input_state: text_input::State,
    pub experience_points_input: i32,
    pub experience_points_input_state: text_input::State,
    pub ability_points_input: i32,
    pub ability_points_input_state: text_input::State,
    pub skin_selectors: SkinSelectors,
    pub gear_unlocker: GearUnlocker,
    pub ammo_setter: AmmoSetter,
    pub sdu_unlocker: SduUnlocker,
}

#[derive(Debug, Clone)]
pub enum SaveCharacterInteractionMessage {
    Name(String),
    Level(i32),
    ExperiencePoints(i32),
    AbilityPoints(i32),
    PlayerClassSelected(PlayerClass),
    SkinMessage(CharacterSkinSelectedMessage),
    GearMessage(CharacterGearUnlockedMessage),
    SduMessage(CharacterSduMessage),
    AmmoMessage(CharacterAmmoMessage),
    MaxSduSlotsPressed,
    MaxAmmoAmountsPressed,
}

#[derive(Debug, Default)]
pub struct CharacterGearState {
    pub unlock_grenade_slot: bool,
    pub unlock_shield_slot: bool,
    pub unlock_weapon_1_slot: bool,
    pub unlock_weapon_2_slot: bool,
    pub unlock_weapon_3_slot: bool,
    pub unlock_weapon_4_slot: bool,
    pub unlock_artifact_slot: bool,
    pub unlock_class_mod_slot: bool,
}

#[derive(Debug, Clone)]
pub enum CharacterSkinSelectedMessage {
    HeadSkin(GameDataKv),
    CharacterSkin(GameDataKv),
    EchoTheme(GameDataKv),
}

#[derive(Debug, Clone)]
pub enum CharacterGearUnlockedMessage {
    Grenade(bool),
    Shield(bool),
    Weapon1(bool),
    Weapon2(bool),
    Weapon3(bool),
    Weapon4(bool),
    Artifact(bool),
    ClassMod(bool),
}

#[derive(Debug, Clone)]
pub enum CharacterSduMessage {
    Backpack(i32),
    Sniper(i32),
    Shotgun(i32),
    Pistol(i32),
    Grenade(i32),
    Smg(i32),
    AssaultRifle(i32),
    Heavy(i32),
}

#[derive(Debug, Clone)]
pub enum CharacterAmmoMessage {
    Sniper(i32),
    Shotgun(i32),
    Pistol(i32),
    Grenade(i32),
    Smg(i32),
    AssaultRifle(i32),
    Heavy(i32),
}

pub fn view(character_state: &mut CharacterState) -> Container<Bl3Message> {
    let selected_class = character_state.player_class_selected_class;

    let character_name = Container::new(
        LabelledElement::create(
            "Name",
            Length::Units(75),
            TextInputLimited::new(
                &mut character_state.name_input_state,
                "FL4K",
                &character_state.name_input,
                500,
                |c| {
                    InteractionMessage::ManageSaveInteraction(
                        ManageSaveInteractionMessage::Character(
                            SaveCharacterInteractionMessage::Name(c),
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
        .align_items(Align::Center),
    )
    .width(Length::FillPortion(3))
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let player_class = Container::new(
        LabelledElement::create(
            "Class",
            Length::Units(65),
            PickList::new(
                &mut character_state.player_class_selector,
                &PlayerClass::ALL[..],
                Some(selected_class),
                |c| {
                    InteractionMessage::ManageSaveInteraction(
                        ManageSaveInteractionMessage::Character(
                            SaveCharacterInteractionMessage::PlayerClassSelected(c),
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
        .align_items(Align::Center),
    )
    .width(Length::FillPortion(1))
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let name_class_row = Row::new()
        .push(character_name)
        .push(player_class)
        .spacing(20);

    let level = Container::new(
        LabelledElement::create(
            "Level",
            Length::Units(60),
            Tooltip::new(
                NumberInput::new(
                    &mut character_state.xp_level_input_state,
                    character_state.level_input,
                    1,
                    Some(MAX_CHARACTER_LEVEL as i32),
                    |v| {
                        InteractionMessage::ManageSaveInteraction(
                            ManageSaveInteractionMessage::Character(
                                SaveCharacterInteractionMessage::Level(v),
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
                format!("Level must be between 1 and {}", MAX_CHARACTER_LEVEL),
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

    let experience_points = Container::new(
        LabelledElement::create(
            "Experience",
            Length::Units(95),
            Tooltip::new(
                NumberInput::new(
                    &mut character_state.experience_points_input_state,
                    character_state.experience_points_input,
                    0,
                    Some(REQUIRED_XP_LIST[MAX_CHARACTER_LEVEL - 1][0]),
                    |v| {
                        InteractionMessage::ManageSaveInteraction(
                            ManageSaveInteractionMessage::Character(
                                SaveCharacterInteractionMessage::ExperiencePoints(v),
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
                "Experience must be between 0 and 9,520,932",
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

    let ability_points = Container::new(
        LabelledElement::create(
            "Skill Points",
            Length::Units(130),
            NumberInput::new(
                &mut character_state.ability_points_input_state,
                character_state.ability_points_input,
                0,
                Some(i32::MAX),
                |v| {
                    InteractionMessage::ManageSaveInteraction(
                        ManageSaveInteractionMessage::Character(
                            SaveCharacterInteractionMessage::AbilityPoints(v),
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

    let experience_and_level_row = Row::new()
        .push(level)
        .push(experience_points)
        .push(ability_points)
        .spacing(20);

    let skin_unlocker = character_state.skin_selectors.view(&selected_class);

    let gear_unlocker = character_state
        .gear_unlocker
        .view()
        .width(Length::FillPortion(3));

    let ammo_setter = character_state
        .ammo_setter
        .view()
        .width(Length::FillPortion(2));

    let sdu_unlocker = character_state
        .sdu_unlocker
        .view()
        .width(Length::FillPortion(2));

    let slot_sdu_row = Row::new()
        .push(gear_unlocker)
        .push(ammo_setter)
        .push(sdu_unlocker)
        .spacing(20);

    let all_contents = Column::new()
        .push(name_class_row)
        .push(experience_and_level_row)
        .push(skin_unlocker)
        .push(slot_sdu_row)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
