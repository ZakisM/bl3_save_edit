use std::rc::Rc;

use derivative::Derivative;
use iced::{Align, Checkbox, Color, Column, Container, Element, Length, Text};

use bl3_save_edit_core::vehicle_data::{VehicleData, VehicleSubType, VehicleType};

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::vehicle::{SaveVehicleInteractionMessage, VehicleUnlockedMessage};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct VehicleUnlockCheckbox {
    name: String,
    pub vehicle_data: VehicleData,
    pub is_unlocked: bool,
    #[derivative(
        Debug = "ignore",
        Default(value = "Rc::new(VehicleUnlockedMessage::OutrunnerChassis)")
    )]
    on_checked: Rc<dyn Fn(bool) -> VehicleUnlockedMessage>,
}

impl VehicleUnlockCheckbox {
    pub fn new<S, F>(name: S, vehicle_data: VehicleData, on_checked: F) -> Self
    where
        S: AsRef<str>,
        F: 'static + Fn(bool) -> VehicleUnlockedMessage,
    {
        VehicleUnlockCheckbox {
            name: name.as_ref().to_owned(),
            vehicle_data,
            is_unlocked: false,
            on_checked: Rc::new(on_checked),
        }
    }

    pub fn view(&mut self) -> Element<Bl3Message> {
        let on_checked = self.on_checked.clone();

        Checkbox::new(
            self.is_unlocked,
            format!(
                "{} [{}/{}]",
                &self.name,
                self.vehicle_data.current,
                self.vehicle_data.vehicle_type.maximum()
            ),
            move |c| {
                InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::Vehicle(
                    SaveVehicleInteractionMessage::UnlockMessage(on_checked(c)),
                ))
            },
        )
        .size(20)
        .font(JETBRAINS_MONO)
        .text_color(Color::from_rgb8(220, 220, 220))
        .text_size(17)
        .style(Bl3UiStyle)
        .into_element()
    }
}

#[derive(Debug)]
pub struct VehicleUnlocker {
    pub outrunner_chassis: VehicleUnlockCheckbox,
    pub outrunner_parts: VehicleUnlockCheckbox,
    pub outrunner_skins: VehicleUnlockCheckbox,
    pub jetbeast_chassis: VehicleUnlockCheckbox,
    pub jetbeast_parts: VehicleUnlockCheckbox,
    pub jetbeast_skins: VehicleUnlockCheckbox,
    pub technical_chassis: VehicleUnlockCheckbox,
    pub technical_parts: VehicleUnlockCheckbox,
    pub technical_skins: VehicleUnlockCheckbox,
    pub cyclone_chassis: VehicleUnlockCheckbox,
    pub cyclone_parts: VehicleUnlockCheckbox,
    pub cyclone_skins: VehicleUnlockCheckbox,
}

impl std::default::Default for VehicleUnlocker {
    fn default() -> Self {
        Self {
            outrunner_chassis: VehicleUnlockCheckbox::new(
                "Unlock All Outrunner Chassis (Wheels)",
                VehicleData::new(VehicleType::Outrunner(VehicleSubType::Chassis), 0),
                VehicleUnlockedMessage::OutrunnerChassis,
            ),
            outrunner_parts: VehicleUnlockCheckbox::new(
                "Unlock All Outrunner Parts",
                VehicleData::new(VehicleType::Outrunner(VehicleSubType::Parts), 0),
                VehicleUnlockedMessage::OutrunnerParts,
            ),
            outrunner_skins: VehicleUnlockCheckbox::new(
                "Unlock All Outrunner Skins",
                VehicleData::new(VehicleType::Outrunner(VehicleSubType::Skins), 0),
                VehicleUnlockedMessage::OutrunnerSkins,
            ),
            jetbeast_chassis: VehicleUnlockCheckbox::new(
                "Unlock All Jetbeast Chassis (Wheels)",
                VehicleData::new(VehicleType::Jetbeast(VehicleSubType::Chassis), 0),
                VehicleUnlockedMessage::JetbeastChassis,
            ),
            jetbeast_parts: VehicleUnlockCheckbox::new(
                "Unlock All Jetbeast Parts",
                VehicleData::new(VehicleType::Jetbeast(VehicleSubType::Parts), 0),
                VehicleUnlockedMessage::JetbeastParts,
            ),
            jetbeast_skins: VehicleUnlockCheckbox::new(
                "Unlock All Jetbeast Skins",
                VehicleData::new(VehicleType::Jetbeast(VehicleSubType::Skins), 0),
                VehicleUnlockedMessage::JetbeastSkins,
            ),
            technical_chassis: VehicleUnlockCheckbox::new(
                "Unlock All Technical Chassis (Wheels)",
                VehicleData::new(VehicleType::Technical(VehicleSubType::Chassis), 0),
                VehicleUnlockedMessage::TechnicalChassis,
            ),
            technical_parts: VehicleUnlockCheckbox::new(
                "Unlock All Technical Parts",
                VehicleData::new(VehicleType::Technical(VehicleSubType::Parts), 0),
                VehicleUnlockedMessage::TechnicalParts,
            ),
            technical_skins: VehicleUnlockCheckbox::new(
                "Unlock All Technical Skins",
                VehicleData::new(VehicleType::Technical(VehicleSubType::Skins), 0),
                VehicleUnlockedMessage::TechnicalSkins,
            ),
            cyclone_chassis: VehicleUnlockCheckbox::new(
                "Unlock All Cyclone Chassis (Wheels)",
                VehicleData::new(VehicleType::Cyclone(VehicleSubType::Chassis), 0),
                VehicleUnlockedMessage::CycloneChassis,
            ),
            cyclone_parts: VehicleUnlockCheckbox::new(
                "Unlock All Cyclone Parts",
                VehicleData::new(VehicleType::Cyclone(VehicleSubType::Parts), 0),
                VehicleUnlockedMessage::CycloneParts,
            ),
            cyclone_skins: VehicleUnlockCheckbox::new(
                "Unlock All Cyclone Skins",
                VehicleData::new(VehicleType::Cyclone(VehicleSubType::Skins), 0),
                VehicleUnlockedMessage::CycloneSkins,
            ),
        }
    }
}

impl VehicleUnlocker {
    pub fn view(&mut self) -> Container<Bl3Message> {
        Container::new(
            Column::new()
                .push(
                    Container::new(
                        Text::new("Vehicle Unlocker")
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
                            .push(self.outrunner_chassis.view())
                            .push(self.outrunner_parts.view())
                            .push(self.outrunner_skins.view())
                            .push(self.jetbeast_chassis.view())
                            .push(self.jetbeast_parts.view())
                            .push(self.jetbeast_skins.view())
                            .push(self.technical_chassis.view())
                            .push(self.technical_parts.view())
                            .push(self.technical_skins.view())
                            .push(self.cyclone_chassis.view())
                            .push(self.cyclone_parts.view())
                            .push(self.cyclone_skins.view())
                            .spacing(15),
                    )
                    .width(Length::Fill)
                    .padding(15)
                    .height(Length::Units(440))
                    .style(Bl3UiStyle),
                ),
        )
    }
}
