use std::rc::Rc;

use derivative::Derivative;
use iced::alignment::{Horizontal, Vertical};
use iced::{
    button, text_input, tooltip, Alignment, Button, Color, Column, Container, Length, Row, Text,
    Tooltip,
};

use bl3_save_edit_core::bl3_profile::sdu::ProfileSduSlot;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_profile::profile::{ProfileProfileInteractionMessage, ProfileSduMessage};
use crate::views::manage_profile::ManageProfileInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_margin::TextMargin;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct SduUnlockField {
    name: String,
    text_margin: usize,
    pub sdu_slot: ProfileSduSlot,
    pub input: i32,
    input_state: text_input::State,
    #[derivative(Debug = "ignore", Default(value = "Rc::new(ProfileSduMessage::Bank)"))]
    on_changed: Rc<dyn Fn(i32) -> ProfileSduMessage>,
}

impl SduUnlockField {
    pub fn new<S, F>(
        name: S,
        text_margin: usize,
        sdu_slot_type: ProfileSduSlot,
        on_changed: F,
    ) -> Self
    where
        S: AsRef<str>,
        F: 'static + Fn(i32) -> ProfileSduMessage,
    {
        SduUnlockField {
            name: name.as_ref().to_owned(),
            text_margin,
            sdu_slot: sdu_slot_type,
            on_changed: Rc::new(on_changed),
            ..Default::default()
        }
    }

    pub fn view(&mut self) -> Row<Bl3Message> {
        let on_changed = self.on_changed.clone();
        let minimum = 0;
        let maximum = self.sdu_slot.maximum();

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
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Profile(
                                    ProfileProfileInteractionMessage::SduMessage(on_changed(v)),
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
            .align_items(Alignment::Center)
    }
}

#[derive(Debug)]
pub struct SduUnlocker {
    pub bank: SduUnlockField,
    pub lost_loot: SduUnlockField,
    unlock_all_button_state: button::State,
}

impl std::default::Default for SduUnlocker {
    fn default() -> Self {
        Self {
            bank: SduUnlockField::new("Bank", 0, ProfileSduSlot::Bank, ProfileSduMessage::Bank),
            lost_loot: SduUnlockField::new(
                "Lost Loot",
                4,
                ProfileSduSlot::LostLoot,
                ProfileSduMessage::LostLoot,
            ),
            unlock_all_button_state: button::State::default(),
        }
    }
}

impl SduUnlocker {
    pub fn view(&mut self) -> Container<Bl3Message> {
        Container::new(
            Column::new()
                .push(
                    Container::new(
                        Text::new("SDU Management")
                            .font(JETBRAINS_MONO_BOLD)
                            .size(17)
                            .color(Color::from_rgb8(242, 203, 5)),
                    )
                    .padding(10)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
                    .style(Bl3UiStyle),
                )
                .push(
                    Container::new(
                        Column::new()
                            .push(
                                Row::new()
                                    .push(self.bank.view())
                                    .push(self.lost_loot.view()),
                            )
                            .push(
                                Container::new(
                                    Button::new(
                                        &mut self.unlock_all_button_state,
                                        Text::new("Max All SDU Levels")
                                            .font(JETBRAINS_MONO_BOLD)
                                            .size(17),
                                    )
                                    .on_press(InteractionMessage::ManageProfileInteraction(
                                        ManageProfileInteractionMessage::Profile(
                                            ProfileProfileInteractionMessage::MaxSduSlotsPressed,
                                        ),
                                    ))
                                    .padding(10)
                                    .style(Bl3UiStyle)
                                    .into_element(),
                                )
                                .height(Length::Fill)
                                .align_y(Vertical::Bottom)
                                .padding(5),
                            )
                            .align_items(Alignment::Center)
                            .spacing(15),
                    )
                    .padding(20)
                    .height(Length::Units(260))
                    .style(Bl3UiStyle),
                ),
        )
    }
}
