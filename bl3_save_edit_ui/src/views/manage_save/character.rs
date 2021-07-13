use iced::{
    container, pick_list, text_input, tooltip, Align, Checkbox, Color, Column, Container, Length,
    PickList, Row, TextInput, Tooltip,
};

use bl3_save_edit_core::bl3_save::player_class::PlayerClass;
use bl3_save_edit_core::bl3_save::sdu::SaveSduSlot;
use bl3_save_edit_core::bl3_save::util::REQUIRED_XP_LIST;
use bl3_save_edit_core::game_data::{
    GameDataKv, PROFILE_ECHO_THEMES, PROFILE_ECHO_THEMES_DEFAULTS, PROFILE_HEADS,
    PROFILE_HEADS_DEFAULTS, PROFILE_SKINS, PROFILE_SKINS_DEFAULTS,
};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::interaction::InteractionExt;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::{ManageSaveInteractionMessage, ManageSaveMessage};
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_margin::TextMargin;
use crate::{generate_sdu_input, generate_skin_pick_list};

pub const MAX_CHARACTER_LEVEL: usize = 72;

#[derive(Debug, Default)]
pub struct CharacterState {
    pub name_input: String,
    pub name_input_state: text_input::State,
    pub player_class_selector: pick_list::State<PlayerClass>,
    pub player_class_selected_class: PlayerClass,
    pub xp_level_input: i32,
    pub xp_level_input_state: text_input::State,
    pub xp_points_input: i32,
    pub xp_points_input_state: text_input::State,
    pub skin_state: CharacterSkinState,
    pub gear_state: CharacterGearState,
    pub sdu_state: CharacterSduState,
}

#[derive(Debug, Default)]
pub struct CharacterSkinState {
    pub head_skin_selector: pick_list::State<GameDataKv>,
    pub head_skin_selected: GameDataKv,
    pub character_skin_selector: pick_list::State<GameDataKv>,
    pub character_skin_selected: GameDataKv,
    pub echo_theme_selector: pick_list::State<GameDataKv>,
    pub echo_theme_selected: GameDataKv,
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

#[derive(Debug, Default)]
pub struct CharacterSduState {
    pub backpack_input: i32,
    pub backpack_input_state: text_input::State,
    pub sniper_input: i32,
    pub sniper_input_state: text_input::State,
    pub shotgun_input: i32,
    pub shotgun_input_state: text_input::State,
    pub pistol_input: i32,
    pub pistol_input_state: text_input::State,
    pub grenade_input: i32,
    pub grenade_input_state: text_input::State,
    pub smg_input: i32,
    pub smg_input_state: text_input::State,
    pub assault_rifle_input: i32,
    pub assault_rifle_input_state: text_input::State,
    pub heavy_input: i32,
    pub heavy_input_state: text_input::State,
}

#[derive(Debug)]
pub enum CharacterMessage {
    PlayerClassSelected(PlayerClass),
    SkinMessage(CharacterSkinMessage),
    GearMessage(CharacterGearMessage),
}

#[derive(Debug)]
pub enum CharacterSkinMessage {
    HeadSkinSelected(GameDataKv),
    CharacterSkinSelected(GameDataKv),
    EchoThemeSelected(GameDataKv),
}

#[derive(Debug)]
pub enum CharacterGearMessage {
    UnlockGrenadeSlot(bool),
    UnlockShieldSlot(bool),
    UnlockWeapon1Slot(bool),
    UnlockWeapon2Slot(bool),
    UnlockWeapon3Slot(bool),
    UnlockWeapon4Slot(bool),
    UnlockArtifactSlot(bool),
    UnlockClassModSlot(bool),
}

#[derive(Debug, Clone)]
pub enum CharacterInteractionMessage {
    NameInputChanged(String),
    XpLevelInputChanged(i32),
    XpPointsInputChanged(i32),
    SduMessage(CharacterInteractionSduMessage),
}

#[derive(Debug, Clone)]
pub enum CharacterInteractionSduMessage {
    BackpackInputChanged(i32),
    SniperInputChanged(i32),
    ShotgunInputChanged(i32),
    PistolInputChanged(i32),
    GrenadeInputChanged(i32),
    SmgInputChanged(i32),
    AssaultRifleInputChanged(i32),
    HeavyInputChanged(i32),
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
    let selected_class = character_state.player_class_selected_class;

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
                        InteractionMessage::ManageSaveInteraction(
                            ManageSaveInteractionMessage::Character(
                                CharacterInteractionMessage::NameInputChanged(s),
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
                    Some(selected_class),
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
                        character_state.xp_level_input,
                        1,
                        Some(MAX_CHARACTER_LEVEL as i32),
                        |v| {
                            InteractionMessage::ManageSaveInteraction(
                                ManageSaveInteractionMessage::Character(
                                    CharacterInteractionMessage::XpLevelInputChanged(v),
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
                    "Level must be between 1 and 72",
                    tooltip::Position::Top,
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
                        character_state.xp_points_input,
                        0,
                        Some(REQUIRED_XP_LIST[MAX_CHARACTER_LEVEL - 1][0]),
                        |v| {
                            InteractionMessage::ManageSaveInteraction(
                                ManageSaveInteractionMessage::Character(
                                    CharacterInteractionMessage::XpPointsInputChanged(v),
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
                .style(TooltipStyle),
            )
            .spacing(15)
            .align_items(Align::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let xp_row = Row::new().push(xp_level).push(xp_points).spacing(20);

    let head_skin = generate_skin_pick_list!(
        "Head Skin",
        105,
        character_state,
        selected_class,
        PROFILE_HEADS_DEFAULTS,
        PROFILE_HEADS,
        head_skin_selector,
        head_skin_selected,
        CharacterSkinMessage::HeadSkinSelected
    );

    let character_skin = generate_skin_pick_list!(
        "Character Skin",
        135,
        character_state,
        selected_class,
        PROFILE_SKINS_DEFAULTS,
        PROFILE_SKINS,
        character_skin_selector,
        character_skin_selected,
        CharacterSkinMessage::CharacterSkinSelected
    );

    let echo_theme = generate_skin_pick_list!(
        "Echo Theme",
        105,
        character_state,
        PROFILE_ECHO_THEMES_DEFAULTS,
        PROFILE_ECHO_THEMES,
        echo_theme_selector,
        echo_theme_selected,
        CharacterSkinMessage::EchoThemeSelected
    );

    let skin_row = Container::new(
        Column::new()
            .push(Row::new().push(head_skin).push(character_skin).spacing(20))
            .push(echo_theme)
            .spacing(20),
    );

    let gear_unlocker = Container::new(
        Column::new()
            .push(
                Container::new(
                    TextMargin::new("Gear Management", 2)
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
                            Checkbox::new(
                                character_state.gear_state.unlock_grenade_slot,
                                "Grenade",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::GearMessage(
                                            CharacterGearMessage::UnlockGrenadeSlot(b),
                                        ),
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
                                character_state.gear_state.unlock_shield_slot,
                                "Shield",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::GearMessage(
                                            CharacterGearMessage::UnlockShieldSlot(b),
                                        ),
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
                                character_state.gear_state.unlock_weapon_1_slot,
                                "Weapon Slot 1",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::GearMessage(
                                            CharacterGearMessage::UnlockWeapon1Slot(b),
                                        ),
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
                                character_state.gear_state.unlock_weapon_2_slot,
                                "Weapon Slot 2",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::GearMessage(
                                            CharacterGearMessage::UnlockWeapon2Slot(b),
                                        ),
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
                                character_state.gear_state.unlock_weapon_3_slot,
                                "Weapon Slot 3",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::GearMessage(
                                            CharacterGearMessage::UnlockWeapon3Slot(b),
                                        ),
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
                                character_state.gear_state.unlock_weapon_4_slot,
                                "Weapon Slot 4",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::GearMessage(
                                            CharacterGearMessage::UnlockWeapon4Slot(b),
                                        ),
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
                                character_state.gear_state.unlock_artifact_slot,
                                "Artifact",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::GearMessage(
                                            CharacterGearMessage::UnlockArtifactSlot(b),
                                        ),
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
                                character_state.gear_state.unlock_class_mod_slot,
                                "Class Mod",
                                |b| {
                                    Message::ManageSave(ManageSaveMessage::Character(
                                        CharacterMessage::GearMessage(
                                            CharacterGearMessage::UnlockClassModSlot(b),
                                        ),
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
                .padding(15)
                .style(Bl3UiStyle),
            ),
    )
    .width(Length::FillPortion(3));

    let sdu_unlocker = Container::new(
        Column::new()
            .push(
                Container::new(
                    TextMargin::new("SDU Management", 2)
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
                            Row::new()
                                .push(generate_sdu_input!(
                                    "Backpack",
                                    1,
                                    SaveSduSlot::Backpack,
                                    character_state,
                                    backpack_input,
                                    backpack_input_state,
                                    CharacterInteractionSduMessage::BackpackInputChanged
                                ))
                                .push(generate_sdu_input!(
                                    "Sniper",
                                    3,
                                    SaveSduSlot::Sniper,
                                    character_state,
                                    sniper_input,
                                    sniper_input_state,
                                    CharacterInteractionSduMessage::SniperInputChanged
                                )),
                        )
                        .push(
                            Row::new()
                                .push(generate_sdu_input!(
                                    "Heavy",
                                    1,
                                    SaveSduSlot::Heavy,
                                    character_state,
                                    heavy_input,
                                    heavy_input_state,
                                    CharacterInteractionSduMessage::HeavyInputChanged
                                ))
                                .push(generate_sdu_input!(
                                    "Shotgun",
                                    3,
                                    SaveSduSlot::Shotgun,
                                    character_state,
                                    shotgun_input,
                                    shotgun_input_state,
                                    CharacterInteractionSduMessage::ShotgunInputChanged
                                )),
                        )
                        .push(
                            Row::new()
                                .push(generate_sdu_input!(
                                    "Grenade",
                                    1,
                                    SaveSduSlot::Grenade,
                                    character_state,
                                    grenade_input,
                                    grenade_input_state,
                                    CharacterInteractionSduMessage::GrenadeInputChanged
                                ))
                                .push(generate_sdu_input!(
                                    "SMG",
                                    3,
                                    SaveSduSlot::Smg,
                                    character_state,
                                    smg_input,
                                    smg_input_state,
                                    CharacterInteractionSduMessage::SmgInputChanged
                                )),
                        )
                        .push(
                            Row::new()
                                .push(generate_sdu_input!(
                                    "AR",
                                    1,
                                    SaveSduSlot::Ar,
                                    character_state,
                                    assault_rifle_input,
                                    assault_rifle_input_state,
                                    CharacterInteractionSduMessage::AssaultRifleInputChanged
                                ))
                                .push(generate_sdu_input!(
                                    "Pistol",
                                    3,
                                    SaveSduSlot::Pistol,
                                    character_state,
                                    pistol_input,
                                    pistol_input_state,
                                    CharacterInteractionSduMessage::PistolInputChanged
                                )),
                        )
                        .spacing(15),
                )
                .padding(15)
                .style(Bl3UiStyle),
            ),
    )
    .width(Length::FillPortion(2));

    //TODO:
    // Set .invbal_ when setting the skin inside save
    // /game/playercharacters/_customizations/beastmaster/heads/customhead_beastmaster_4.customhead_beastmaster_4
    // /game/playercharacters/_customizations/beastmaster/heads/customhead_beastmaster_4.invbal_customhead_beastmaster_4

    let slot_sdu_row = Row::new()
        .push(gear_unlocker)
        .push(sdu_unlocker)
        .spacing(20);

    let all_contents = Column::new()
        .push(name_class_row)
        .push(xp_row)
        .push(skin_row)
        .push(slot_sdu_row)
        .spacing(20);

    Container::new(all_contents).padding(30)
}

#[macro_export]
macro_rules! generate_skin_pick_list {
    ($name:literal, $name_width:literal, $character_state:path, $default_skin_list:path, $skin_list:path, $selector_state:ident, $selected_skin:ident, $message:path) => {{
        let class_available_skins = $default_skin_list
            .iter()
            .cloned()
            .chain($skin_list)
            .collect::<Vec<_>>();

        generate_skin_pick_list!(
            $name,
            $name_width,
            $character_state,
            class_available_skins,
            $selector_state,
            $selected_skin,
            $message
        )
    }};
    ($name:literal, $name_width:literal, $character_state:path, $selected_class:path, $default_skin_list:path, $skin_list:path, $selector_state:ident, $selected_skin:ident, $message:path) => {{
        let class_available_skins = $default_skin_list
            .iter()
            .cloned()
            .chain($skin_list)
            .filter(|h| {
                h.0 .0
                    .to_lowercase()
                    .contains(&$selected_class.to_string().to_lowercase())
            })
            .collect::<Vec<_>>();

        generate_skin_pick_list!(
            $name,
            $name_width,
            $character_state,
            class_available_skins,
            $selector_state,
            $selected_skin,
            $message
        )
    }};
    ($name:literal, $name_width:literal, $character_state:path, $class_available_skins:ident, $selector_state:ident, $selected_skin:ident, $message:path) => {{
        let mut class_available_skins = $class_available_skins;
        class_available_skins.sort();

        let pre_selected_skin = match $character_state.skin_state.$selected_skin {
            GameDataKv((k, _))
                if k.is_empty()
                    || !class_available_skins
                        .contains(&$character_state.skin_state.$selected_skin) =>
            {
                Some(class_available_skins[0])
            }
            k => Some(k),
        };

        Container::new(
            Row::new()
                .push(
                    TextMargin::new($name, 2)
                        .0
                        .font(JETBRAINS_MONO)
                        .size(17)
                        .color(Color::from_rgb8(242, 203, 5))
                        .width(Length::Units($name_width)),
                )
                .push(
                    PickList::new(
                        &mut $character_state.skin_state.$selector_state,
                        class_available_skins,
                        pre_selected_skin,
                        |selected| {
                            Message::ManageSave(ManageSaveMessage::Character(
                                CharacterMessage::SkinMessage($message(selected)),
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
        .width(Length::Fill)
        .height(Length::Units(36))
        .style(Bl3UiStyle)
    }};
}

#[macro_export]
macro_rules! generate_sdu_input {
    ($name:literal, $text_margin:literal, $sdu_slot_type:path, $character_state:path, $input_value:ident, $input_state:ident, $input_message:path) => {{
        let maximum = $sdu_slot_type.maximum();

        Container::new(
            Row::new()
                .push(
                    TextMargin::new($name, $text_margin)
                        .0
                        .font(JETBRAINS_MONO)
                        .size(17)
                        .color(Color::from_rgb8(220, 220, 220))
                        .width(Length::Units(120)),
                )
                .push(
                    Tooltip::new(
                        NumberInput::new(
                            &mut $character_state.sdu_state.$input_state,
                            $character_state.sdu_state.$input_value,
                            0,
                            Some(maximum),
                            |v| {
                                InteractionMessage::ManageSaveInteraction(
                                    ManageSaveInteractionMessage::Character(
                                        CharacterInteractionMessage::SduMessage($input_message(v)),
                                    ),
                                )
                            },
                        )
                        .0
                        .width(Length::Fill)
                        .font(JETBRAINS_MONO)
                        .padding(10)
                        .size(17)
                        .style(Bl3UiStyle)
                        .into_element(),
                        format!("Level must be between 0 and {}", maximum),
                        tooltip::Position::Top,
                    )
                    .gap(10)
                    .padding(10)
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .style(TooltipStyle),
                )
                .spacing(15)
                .width(Length::Fill)
                .align_items(Align::Center),
        )
        .width(Length::Fill)
    }};
}
