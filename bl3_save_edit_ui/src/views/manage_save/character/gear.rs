use iced::{Align, Checkbox, Color, Column, Container, Length};

use crate::bl3_ui::Message;
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::character::{CharacterMessage, CharacterUnlockGearMessage};
use crate::views::manage_save::ManageSaveMessage;
use crate::widgets::text_margin::TextMargin;

pub fn view() -> Container<Message> {
    Container::new(
        Column::new()
            .push(
                Container::new(
                    TextMargin::new("Gear Management", 2)
                        .0
                        .font(JETBRAINS_MONO_BOLD)
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
                                            CharacterUnlockGearMessage::Grenade(b),
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
                                            CharacterUnlockGearMessage::Shield(b),
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
                                            CharacterUnlockGearMessage::Weapon1(b),
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
                                            CharacterUnlockGearMessage::Weapon2(b),
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
                                            CharacterUnlockGearMessage::Weapon3(b),
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
                                            CharacterUnlockGearMessage::Weapon4(b),
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
                                            CharacterUnlockGearMessage::Artifact(b),
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
                                            CharacterUnlockGearMessage::ClassMod(b),
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
                        .spacing(15),
                )
                .width(Length::Fill)
                .padding(15)
                .style(Bl3UiStyle),
            ),
    )
}
