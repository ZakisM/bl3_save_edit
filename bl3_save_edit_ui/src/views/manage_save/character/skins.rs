use std::rc::Rc;

use derivative::Derivative;
use iced::{pick_list, Alignment, Column, Container, Length, PickList, Row};
use rayon::prelude::ParallelSliceMut;

use bl3_save_edit_core::bl3_save::player_class::PlayerClass;
use bl3_save_edit_core::game_data::{
    GameDataKv, PROFILE_ECHO_THEMES, PROFILE_ECHO_THEMES_DEFAULTS, PROFILE_HEADS,
    PROFILE_HEADS_DEFAULTS, PROFILE_SKINS, PROFILE_SKINS_DEFAULTS,
};

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::character::{
    CharacterSkinSelectedMessage, SaveCharacterInteractionMessage,
};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct SkinPickList {
    name: String,
    name_width: u16,
    available_skins: Vec<GameDataKv>,
    pick_list: pick_list::State<GameDataKv>,
    pub selected: GameDataKv,
    #[derivative(
        Debug = "ignore",
        Default(value = "Rc::new(CharacterSkinSelectedMessage::HeadSkin)")
    )]
    on_selected: Rc<dyn Fn(GameDataKv) -> CharacterSkinSelectedMessage>,
}

impl SkinPickList {
    pub fn new<S, F>(
        name: S,
        name_width: u16,
        default_skin_list: &[GameDataKv],
        skin_list: &[GameDataKv],
        on_selected: F,
    ) -> Self
    where
        S: AsRef<str>,
        F: 'static + Fn(GameDataKv) -> CharacterSkinSelectedMessage,
    {
        let mut available_skins = default_skin_list
            .iter()
            .chain(skin_list.iter())
            .cloned()
            .collect::<Vec<_>>();

        available_skins.par_sort();

        let pre_selected_skin = available_skins[0];

        Self {
            name: name.as_ref().to_owned(),
            name_width,
            available_skins,
            selected: pre_selected_skin,
            pick_list: pick_list::State::default(),
            on_selected: Rc::new(on_selected),
        }
    }

    pub fn view(&mut self, player_class: Option<&PlayerClass>) -> Container<Bl3Message> {
        let on_selected = self.on_selected.clone();

        let available_skins = if let Some(player_class) = player_class {
            let player_class_s = player_class.to_string().to_lowercase();

            self.available_skins
                .iter()
                .cloned()
                .filter(|h| h.ident.to_lowercase().contains(&player_class_s))
                .collect::<Vec<_>>()
        } else {
            self.available_skins.to_vec()
        };

        if !available_skins.contains(&self.selected) {
            self.selected = available_skins[0];
        }

        Container::new(
            LabelledElement::create(
                &self.name,
                Length::Units(self.name_width),
                PickList::new(
                    &mut self.pick_list,
                    available_skins,
                    Some(self.selected),
                    move |s| {
                        InteractionMessage::ManageSaveInteraction(
                            ManageSaveInteractionMessage::Character(
                                SaveCharacterInteractionMessage::SkinMessage(on_selected(s)),
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
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Units(36))
        .style(Bl3UiStyle)
    }
}

#[derive(Debug)]
pub struct SkinSelectors {
    pub head_skin: SkinPickList,
    pub character_skin: SkinPickList,
    pub echo_theme: SkinPickList,
}

impl std::default::Default for SkinSelectors {
    fn default() -> Self {
        Self {
            head_skin: SkinPickList::new(
                "Head Skin",
                105,
                &PROFILE_HEADS_DEFAULTS,
                &PROFILE_HEADS,
                CharacterSkinSelectedMessage::HeadSkin,
            ),
            character_skin: SkinPickList::new(
                "Character Skin",
                135,
                &PROFILE_SKINS_DEFAULTS,
                &PROFILE_SKINS,
                CharacterSkinSelectedMessage::CharacterSkin,
            ),
            echo_theme: SkinPickList::new(
                "ECHO Theme",
                105,
                &PROFILE_ECHO_THEMES_DEFAULTS,
                &PROFILE_ECHO_THEMES,
                CharacterSkinSelectedMessage::EchoTheme,
            ),
        }
    }
}

impl SkinSelectors {
    pub fn view(&mut self, player_class: &PlayerClass) -> Container<Bl3Message> {
        let head_skin = self.head_skin.view(Some(player_class));
        let character_skin = self.character_skin.view(Some(player_class));
        let echo_theme = self.echo_theme.view(None);

        Container::new(
            Column::new()
                .push(Row::new().push(head_skin).push(character_skin).spacing(20))
                .push(echo_theme)
                .spacing(20),
        )
    }
}
