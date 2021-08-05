use std::rc::Rc;

use derivative::Derivative;
use iced::{
    button, text_input, tooltip, Align, Button, Color, Column, Container, Length, Row, Text,
    Tooltip,
};

use bl3_save_edit_core::bl3_save::sdu::SaveSduSlot;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::character::{
    CharacterInteractionMessage, CharacterSduInputChangedMessage,
};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_margin::TextMargin;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct SduUnlockField {
    name: String,
    text_margin: usize,
    sdu_slot_type: SaveSduSlot,
    pub input: i32,
    input_state: text_input::State,
    #[derivative(
        Debug = "ignore",
        Default(value = "Rc::new(CharacterSduInputChangedMessage::Backpack)")
    )]
    on_changed: Rc<dyn Fn(i32) -> CharacterSduInputChangedMessage>,
}

impl SduUnlockField {
    pub fn new<S, F>(name: S, text_margin: usize, sdu_slot_type: SaveSduSlot, on_changed: F) -> Self
    where
        S: AsRef<str>,
        F: 'static + Fn(i32) -> CharacterSduInputChangedMessage,
    {
        SduUnlockField {
            name: name.as_ref().to_owned(),
            text_margin,
            sdu_slot_type,
            input: 0,
            input_state: text_input::State::default(),
            on_changed: Rc::new(on_changed),
        }
    }

    pub fn view(&mut self) -> Row<Message> {
        let on_changed = self.on_changed.clone();
        let minimum = 0;
        let maximum = self.sdu_slot_type.maximum();

        Row::new()
            .push(
                TextMargin::new(&self.name, self.text_margin)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(220, 220, 220))
                    .width(Length::FillPortion(8)),
            )
            .push(
                Tooltip::new(
                    NumberInput::new(
                        &mut self.input_state,
                        self.input,
                        minimum,
                        Some(maximum),
                        move |v| {
                            InteractionMessage::ManageSaveInteraction(
                                ManageSaveInteractionMessage::Character(
                                    CharacterInteractionMessage::SduMessage(on_changed(v)),
                                ),
                            )
                        },
                    )
                    .0
                    .width(Length::FillPortion(3))
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                    format!("Level must be between {} and {}", minimum, maximum),
                    tooltip::Position::Top,
                )
                .gap(10)
                .padding(10)
                .font(JETBRAINS_MONO)
                .size(17)
                .style(Bl3UiTooltipStyle),
            )
            .width(Length::Fill)
            .align_items(Align::Center)
    }
}

#[derive(Debug)]
pub struct SduUnlocker {
    pub backpack: SduUnlockField,
    pub sniper: SduUnlockField,
    pub heavy: SduUnlockField,
    pub shotgun: SduUnlockField,
    pub grenade: SduUnlockField,
    pub smg: SduUnlockField,
    pub assault_rifle: SduUnlockField,
    pub pistol: SduUnlockField,
    unlock_all_button_state: button::State,
}

impl std::default::Default for SduUnlocker {
    fn default() -> Self {
        Self {
            backpack: SduUnlockField::new(
                "Backpack",
                0,
                SaveSduSlot::Backpack,
                CharacterSduInputChangedMessage::Backpack,
            ),
            sniper: SduUnlockField::new(
                "Sniper",
                4,
                SaveSduSlot::Sniper,
                CharacterSduInputChangedMessage::Sniper,
            ),
            heavy: SduUnlockField::new(
                "Heavy",
                0,
                SaveSduSlot::Heavy,
                CharacterSduInputChangedMessage::Heavy,
            ),
            shotgun: SduUnlockField::new(
                "Shotgun",
                4,
                SaveSduSlot::Shotgun,
                CharacterSduInputChangedMessage::Shotgun,
            ),
            grenade: SduUnlockField::new(
                "Grenade",
                0,
                SaveSduSlot::Grenade,
                CharacterSduInputChangedMessage::Grenade,
            ),
            smg: SduUnlockField::new(
                "SMG",
                4,
                SaveSduSlot::Smg,
                CharacterSduInputChangedMessage::Smg,
            ),
            assault_rifle: SduUnlockField::new(
                "AR",
                0,
                SaveSduSlot::Ar,
                CharacterSduInputChangedMessage::AssaultRifle,
            ),
            pistol: SduUnlockField::new(
                "Pistol",
                4,
                SaveSduSlot::Pistol,
                CharacterSduInputChangedMessage::Pistol,
            ),
            unlock_all_button_state: button::State::default(),
        }
    }
}

impl SduUnlocker {
    pub fn view(&mut self) -> Container<Message> {
        Container::new(
            Column::new()
                .push(
                    Container::new(
                        TextMargin::new("SDU Management", 2)
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
                                Row::new()
                                    .push(self.backpack.view())
                                    .push(self.sniper.view()),
                            )
                            .push(Row::new().push(self.heavy.view()).push(self.shotgun.view()))
                            .push(Row::new().push(self.grenade.view()).push(self.smg.view()))
                            .push(
                                Row::new()
                                    .push(self.assault_rifle.view())
                                    .push(self.pistol.view()),
                            )
                            .push(
                                Container::new(
                                    Button::new(
                                        &mut self.unlock_all_button_state,
                                        Text::new("Max All SDU Levels")
                                            .font(JETBRAINS_MONO_BOLD)
                                            .size(17),
                                    )
                                    .on_press(InteractionMessage::ManageSaveInteraction(
                                        ManageSaveInteractionMessage::Character(
                                            CharacterInteractionMessage::MaxSduSlotsPressed,
                                        ),
                                    ))
                                    .padding(10)
                                    .style(Bl3UiStyle)
                                    .into_element(),
                                )
                                .padding(5),
                            )
                            .align_items(Align::Center)
                            .spacing(15),
                    )
                    .padding(20)
                    .style(Bl3UiStyle),
                ),
        )
    }
}