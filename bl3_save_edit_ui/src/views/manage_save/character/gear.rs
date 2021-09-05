use std::rc::Rc;

use derivative::Derivative;
use iced::{Align, Checkbox, Color, Column, Container, Element, Length};

use bl3_save_edit_core::bl3_save::inventory_slot::InventorySlot;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::character::{
    CharacterGearUnlockedMessage, SaveCharacterInteractionMessage,
};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::text_margin::TextMargin;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct GearUnlockCheckbox {
    name: String,
    pub inv_slot: InventorySlot,
    pub is_unlocked: bool,
    #[derivative(
        Debug = "ignore",
        Default(value = "Rc::new(CharacterGearUnlockedMessage::Grenade)")
    )]
    on_checked: Rc<dyn Fn(bool) -> CharacterGearUnlockedMessage>,
}

impl GearUnlockCheckbox {
    pub fn new<S, F>(name: S, on_checked: F) -> Self
    where
        S: AsRef<str>,
        F: 'static + Fn(bool) -> CharacterGearUnlockedMessage,
    {
        GearUnlockCheckbox {
            name: name.as_ref().to_owned(),
            inv_slot: InventorySlot::default(),
            is_unlocked: false,
            on_checked: Rc::new(on_checked),
        }
    }

    pub fn view(&mut self) -> Element<Bl3Message> {
        let on_checked = self.on_checked.clone();

        Checkbox::new(self.is_unlocked, &self.name, move |c| {
            InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::Character(
                SaveCharacterInteractionMessage::GearMessage(on_checked(c)),
            ))
        })
        .size(20)
        .font(JETBRAINS_MONO)
        .text_color(Color::from_rgb8(220, 220, 220))
        .text_size(17)
        .style(Bl3UiStyle)
        .into_element()
    }
}

#[derive(Debug)]
pub struct GearUnlocker {
    pub grenade: GearUnlockCheckbox,
    pub shield: GearUnlockCheckbox,
    pub weapon_1: GearUnlockCheckbox,
    pub weapon_2: GearUnlockCheckbox,
    pub weapon_3: GearUnlockCheckbox,
    pub weapon_4: GearUnlockCheckbox,
    pub artifact: GearUnlockCheckbox,
    pub class_mod: GearUnlockCheckbox,
}

impl std::default::Default for GearUnlocker {
    fn default() -> Self {
        Self {
            grenade: GearUnlockCheckbox::new("Grenade", CharacterGearUnlockedMessage::Grenade),
            shield: GearUnlockCheckbox::new("Shield", CharacterGearUnlockedMessage::Shield),
            weapon_1: GearUnlockCheckbox::new(
                "Weapon Slot 1",
                CharacterGearUnlockedMessage::Weapon1,
            ),
            weapon_2: GearUnlockCheckbox::new(
                "Weapon Slot 2",
                CharacterGearUnlockedMessage::Weapon2,
            ),
            weapon_3: GearUnlockCheckbox::new(
                "Weapon Slot 3",
                CharacterGearUnlockedMessage::Weapon3,
            ),
            weapon_4: GearUnlockCheckbox::new(
                "Weapon Slot 4",
                CharacterGearUnlockedMessage::Weapon4,
            ),
            artifact: GearUnlockCheckbox::new("Artifact", CharacterGearUnlockedMessage::Artifact),
            class_mod: GearUnlockCheckbox::new("Class Mod", CharacterGearUnlockedMessage::ClassMod),
        }
    }
}

impl GearUnlocker {
    pub fn view(&mut self) -> Container<Bl3Message> {
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
                            .push(self.grenade.view())
                            .push(self.shield.view())
                            .push(self.weapon_1.view())
                            .push(self.weapon_2.view())
                            .push(self.weapon_3.view())
                            .push(self.weapon_4.view())
                            .push(self.artifact.view())
                            .push(self.class_mod.view())
                            .spacing(15),
                    )
                    .width(Length::Fill)
                    .padding(15)
                    .style(Bl3UiStyle),
                ),
        )
    }
}
