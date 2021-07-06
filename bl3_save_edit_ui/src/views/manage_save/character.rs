use std::ops::Deref;

use iced::{
    container, pick_list, text_input, tooltip, Align, Checkbox, Color, Column, Container,
    HorizontalAlignment, Length, PickList, Row, Rule, Text, TextInput, Tooltip,
};

use bl3_save_edit_core::bl3_save::player_class::PlayerClass;
use bl3_save_edit_core::bl3_save::util::REQUIRED_XP_LIST;

use crate::bl3_ui::{Bl3Ui, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::ManageSaveMessage;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_margin::TextMargin;

pub const MAX_CHARACTER_LEVEL: usize = 72;

#[derive(Debug, Default)]
pub struct CharacterState {
    pub name_input: String,
    pub name_input_state: text_input::State,
    pub player_class_selector: pick_list::State<PlayerClass>,
    pub player_class_selected_class: PlayerClass,
    pub xp_level_input: usize,
    pub xp_level_input_state: text_input::State,
    pub xp_points_input: usize,
    pub xp_points_input_state: text_input::State,
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
pub enum CharacterMessage {
    CharacterNameInputChanged(String),
    PlayerClassSelected(PlayerClass),
    XpLevelInputChanged(usize),
    XpPointsInputChanged(usize),
    UnlockGrenadeSlot(bool),
    UnlockShieldSlot(bool),
    UnlockWeapon1Slot(bool),
    UnlockWeapon2Slot(bool),
    UnlockWeapon3Slot(bool),
    UnlockWeapon4Slot(bool),
    UnlockArtifactSlot(bool),
    UnlockClassModSlot(bool),
}

struct TooltipStyle;

impl container::StyleSheet for TooltipStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(220, 220, 220)),
            background: Color::from_rgb8(35, 35, 35).into(),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
        }
    }
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
                    .width(Length::Units(65)),
            )
            .push(
                TextInput::new(
                    &mut character_state.name_input_state,
                    "FL4K",
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

    let player_class = Container::new(
        Row::new()
            .push(
                TextMargin::new("Class", 2)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(242, 203, 5))
                    .width(Length::Units(65)),
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

    let name_class_row = Row::new()
        .push(character_name)
        .push(player_class)
        .spacing(20);

    let xp_level = Container::new(
        Row::new()
            .push(
                TextMargin::new("Level", 2)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(242, 203, 5))
                    .width(Length::Units(55)),
            )
            .push(
                Tooltip::new(
                    NumberInput::new(
                        &mut character_state.xp_level_input_state,
                        "1",
                        character_state.xp_level_input,
                        Some(MAX_CHARACTER_LEVEL),
                        |v| {
                            Message::ManageSave(ManageSaveMessage::Character(
                                CharacterMessage::XpLevelInputChanged(v),
                            ))
                        },
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle),
                    "Level must be between 1 and 72",
                    tooltip::Position::Bottom,
                )
                .gap(10)
                .padding(10)
                .font(JETBRAINS_MONO)
                .size(17)
                .style(TooltipStyle),
            )
            .spacing(15)
            .align_items(Align::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let xp_points = Container::new(
        Row::new()
            .push(
                TextMargin::new("Experience", 2)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(242, 203, 5))
                    .width(Length::Units(95)),
            )
            .push(
                Tooltip::new(
                    NumberInput::new(
                        &mut character_state.xp_points_input_state,
                        "0",
                        character_state.xp_points_input,
                        Some(REQUIRED_XP_LIST[MAX_CHARACTER_LEVEL - 1][0] as usize),
                        |v| {
                            Message::ManageSave(ManageSaveMessage::Character(
                                CharacterMessage::XpPointsInputChanged(v),
                            ))
                        },
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle),
                    "Experience must be between 0 and 9,520,932",
                    tooltip::Position::Bottom,
                )
                .gap(10)
                .padding(10)
                .font(JETBRAINS_MONO)
                .size(17)
                .style(TooltipStyle),
            )
            .spacing(15)
            .align_items(Align::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let xp_row = Row::new().push(xp_level).push(xp_points).spacing(20);

    let slot_unlocker = Container::new(
        Column::new()
            .push(
                Container::new(
                    TextMargin::new("Slots Management", 2)
                        .0
                        .font(JETBRAINS_MONO)
                        .size(17)
                        .color(Color::from_rgb8(242, 203, 5)),
                )
                .padding(10)
                .align_x(Align::Center)
                .width(Length::Fill)
                .style(Bl3UiStyle),
            )
            .push(
                Container::new(
                    Column::new()
                        .push(
                            Checkbox::new(character_state.unlock_grenade_slot, "Grenade", |b| {
                                Message::ManageSave(ManageSaveMessage::Character(
                                    CharacterMessage::UnlockGrenadeSlot(b),
                                ))
                            })
                            .size(20)
                            .font(JETBRAINS_MONO)
                            .text_color(Color::from_rgb8(220, 220, 220))
                            .text_size(17)
                            .style(Bl3UiStyle),
                        )
                        .push(
                            Checkbox::new(character_state.unlock_shield_slot, "Shield", |b| {
                                Message::ManageSave(ManageSaveMessage::Character(
                                    CharacterMessage::UnlockShieldSlot(b),
                                ))
                            })
                            .size(20)
                            .font(JETBRAINS_MONO)
                            .text_color(Color::from_rgb8(220, 220, 220))
                            .text_size(17)
                            .style(Bl3UiStyle),
                        )
                        .push(
                            Checkbox::new(
                                character_state.unlock_weapon_1_slot,
                                "Weapon Slot 1",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::UnlockWeapon1Slot(b),
                                    ))
                                },
                            )
                            .size(20)
                            .font(JETBRAINS_MONO)
                            .text_color(Color::from_rgb8(220, 220, 220))
                            .text_size(17)
                            .style(Bl3UiStyle),
                        )
                        .push(
                            Checkbox::new(
                                character_state.unlock_weapon_2_slot,
                                "Weapon Slot 2",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::UnlockWeapon2Slot(b),
                                    ))
                                },
                            )
                            .size(20)
                            .font(JETBRAINS_MONO)
                            .text_color(Color::from_rgb8(220, 220, 220))
                            .text_size(17)
                            .style(Bl3UiStyle),
                        )
                        .push(
                            Checkbox::new(
                                character_state.unlock_weapon_3_slot,
                                "Weapon Slot 3",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::UnlockWeapon3Slot(b),
                                    ))
                                },
                            )
                            .size(20)
                            .font(JETBRAINS_MONO)
                            .text_color(Color::from_rgb8(220, 220, 220))
                            .text_size(17)
                            .style(Bl3UiStyle),
                        )
                        .push(
                            Checkbox::new(
                                character_state.unlock_weapon_4_slot,
                                "Weapon Slot 4",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::UnlockWeapon4Slot(b),
                                    ))
                                },
                            )
                            .size(20)
                            .font(JETBRAINS_MONO)
                            .text_color(Color::from_rgb8(220, 220, 220))
                            .text_size(17)
                            .style(Bl3UiStyle),
                        )
                        .push(
                            Checkbox::new(character_state.unlock_artifact_slot, "Artifact", |b| {
                                Message::ManageSave(ManageSaveMessage::Character(
                                    CharacterMessage::UnlockArtifactSlot(b),
                                ))
                            })
                            .size(20)
                            .font(JETBRAINS_MONO)
                            .text_color(Color::from_rgb8(220, 220, 220))
                            .text_size(17)
                            .style(Bl3UiStyle),
                        )
                        .push(
                            Checkbox::new(
                                character_state.unlock_class_mod_slot,
                                "Class Mod",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::UnlockClassModSlot(b),
                                    ))
                                },
                            )
                            .size(20)
                            .font(JETBRAINS_MONO)
                            .text_color(Color::from_rgb8(220, 220, 220))
                            .text_size(17)
                            .style(Bl3UiStyle),
                        )
                        .spacing(20),
                )
                .width(Length::Fill)
                .padding(10)
                .style(Bl3UiStyle),
            ),
    )
    .width(Length::Fill);

    //TODO:
    // Set .invbal_ when setting the skin inside save
    // /game/playercharacters/_customizations/beastmaster/heads/customhead_beastmaster_4.customhead_beastmaster_4
    // /game/playercharacters/_customizations/beastmaster/heads/customhead_beastmaster_4.invbal_customhead_beastmaster_4

    let skin_row = Row::new().push(slot_unlocker).spacing(20);

    let all_contents = Column::new()
        .push(name_class_row)
        .push(xp_row)
        .push(skin_row)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
