use std::rc::Rc;

use derivative::Derivative;
use iced::{pick_list, Align, Container, Length, PickList};

use bl3_save_edit_core::bl3_save::player_class::PlayerClass;
use bl3_save_edit_core::game_data::GameDataKv;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::manage_save::character::{
    CharacterInteractionMessage, CharacterSkinSelectedMessage,
};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SkinPickList {
    name: String,
    name_width: u16,
    available_skins: Vec<GameDataKv>,
    pick_list: pick_list::State<GameDataKv>,
    selected: Rc<GameDataKv>,
    #[derivative(Debug = "ignore")]
    on_selected: Rc<dyn Fn(GameDataKv) -> CharacterSkinSelectedMessage>,
}

impl std::default::Default for SkinPickList {
    fn default() -> Self {
        Self {
            name: Default::default(),
            name_width: Default::default(),
            available_skins: Default::default(),
            pick_list: Default::default(),
            selected: Default::default(),
            on_selected: Rc::new(CharacterSkinSelectedMessage::HeadSkin),
        }
    }
}

impl SkinPickList {
    pub fn new<S, F>(
        name: S,
        name_width: u16,
        default_skin_list: &[GameDataKv],
        skin_list: &[GameDataKv],
        selected_skin: &mut GameDataKv,
        selected_class: Option<PlayerClass>,
        on_selected: F,
    ) -> Self
    where
        S: AsRef<str>,
        F: 'static + Fn(GameDataKv) -> CharacterSkinSelectedMessage,
    {
        let mut available_skins = if let Some(selected_class) = selected_class {
            let selected_class_s = selected_class.to_string().to_lowercase();

            default_skin_list
                .iter()
                .chain(skin_list.iter())
                .cloned()
                .filter(|h| h.ident.to_lowercase().contains(&selected_class_s))
                .collect::<Vec<_>>()
        } else {
            default_skin_list
                .iter()
                .chain(skin_list.iter())
                .cloned()
                .collect::<Vec<_>>()
        };

        available_skins.sort();

        if !available_skins.contains(selected_skin) {
            *selected_skin = available_skins[0];
        }

        Self {
            name: name.as_ref().to_owned(),
            name_width,
            available_skins,
            selected: Rc::new(*selected_skin),
            pick_list: pick_list::State::default(),
            on_selected: Rc::new(on_selected),
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        let on_selected = self.on_selected.clone();

        Container::new(
            LabelledElement::create(
                &self.name,
                Length::Units(self.name_width),
                PickList::new(
                    &mut self.pick_list,
                    &self.available_skins,
                    Some(*self.selected),
                    move |s| {
                        InteractionMessage::ManageSaveInteraction(
                            ManageSaveInteractionMessage::Character(
                                CharacterInteractionMessage::SkinMessage(on_selected(s)),
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
        .width(Length::Fill)
        .height(Length::Units(36))
        .style(Bl3UiStyle)
    }
}
