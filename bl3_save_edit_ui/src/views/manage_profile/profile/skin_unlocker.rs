use std::rc::Rc;

use derivative::Derivative;
use iced::{Align, Checkbox, Color, Column, Container, Element, Length};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_profile::profile::{
    ProfileProfileInteractionMessage, ProfileSkinUnlockedMessage,
};
use crate::views::manage_profile::ManageProfileInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::text_margin::TextMargin;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct SkinUnlockCheckbox {
    name: String,
    pub is_unlocked: bool,
    #[derivative(
        Debug = "ignore",
        Default(value = "Rc::new(ProfileSkinUnlockedMessage::CharacterSkins)")
    )]
    on_checked: Rc<dyn Fn(bool) -> ProfileSkinUnlockedMessage>,
}

impl SkinUnlockCheckbox {
    pub fn new<S, F>(name: S, on_checked: F) -> Self
    where
        S: AsRef<str>,
        F: 'static + Fn(bool) -> ProfileSkinUnlockedMessage,
    {
        SkinUnlockCheckbox {
            name: name.as_ref().to_owned(),
            is_unlocked: false,
            on_checked: Rc::new(on_checked),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let on_checked = self.on_checked.clone();

        Checkbox::new(self.is_unlocked, &self.name, move |c| {
            InteractionMessage::ManageProfileInteraction(ManageProfileInteractionMessage::Profile(
                ProfileProfileInteractionMessage::SkinMessage(on_checked(c)),
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
pub struct SkinUnlocker {
    pub character_skins: SkinUnlockCheckbox,
    pub character_heads: SkinUnlockCheckbox,
    pub echo_themes: SkinUnlockCheckbox,
    pub emotes: SkinUnlockCheckbox,
    pub room_decorations: SkinUnlockCheckbox,
    pub weapon_skins: SkinUnlockCheckbox,
    pub weapon_trinkets: SkinUnlockCheckbox,
}

impl std::default::Default for SkinUnlocker {
    fn default() -> Self {
        Self {
            character_skins: SkinUnlockCheckbox::new(
                "Character Skins",
                ProfileSkinUnlockedMessage::CharacterSkins,
            ),
            character_heads: SkinUnlockCheckbox::new(
                "Character Heads",
                ProfileSkinUnlockedMessage::CharacterHeads,
            ),
            echo_themes: SkinUnlockCheckbox::new(
                "Echo Themes",
                ProfileSkinUnlockedMessage::EchoThemes,
            ),
            emotes: SkinUnlockCheckbox::new("Emotes", ProfileSkinUnlockedMessage::Emotes),
            room_decorations: SkinUnlockCheckbox::new(
                "Room Decorations",
                ProfileSkinUnlockedMessage::RoomDecorations,
            ),
            weapon_skins: SkinUnlockCheckbox::new(
                "Weapon Skins",
                ProfileSkinUnlockedMessage::WeaponSkins,
            ),
            weapon_trinkets: SkinUnlockCheckbox::new(
                "Weapon Trinkets",
                ProfileSkinUnlockedMessage::WeaponTrinkets,
            ),
        }
    }
}

impl SkinUnlocker {
    pub fn view(&mut self) -> Container<Message> {
        Container::new(
            Column::new()
                .push(
                    Container::new(
                        TextMargin::new("Skin Unlocker", 2)
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
                            .push(self.character_skins.view())
                            .push(self.character_heads.view())
                            .push(self.echo_themes.view())
                            .push(self.emotes.view())
                            .push(self.room_decorations.view())
                            .push(self.weapon_skins.view())
                            .push(self.weapon_trinkets.view())
                            .spacing(15),
                    )
                    .width(Length::Fill)
                    .padding(15)
                    .height(Length::Units(260))
                    .style(Bl3UiStyle),
                ),
        )
    }
}
