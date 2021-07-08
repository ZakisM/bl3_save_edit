use std::path::PathBuf;

use anyhow::Result;
use iced::{button, Button, Column, Container, HorizontalAlignment, Text};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::interaction::InteractionExt;
use crate::resources::fonts::JETBRAINS_MONO_BOLD;

#[derive(Debug, Default)]
pub struct ChooseSaveDirectoryState {
    choose_dir_button_state: button::State,
}

#[derive(Debug)]
pub enum ChooseSaveMessage {
    ChooseDirCompleted(Result<PathBuf>),
}

#[derive(Debug, Clone)]
pub enum ChooseSaveInteractionMessage {
    ChooseDirPressed,
}

pub fn view(choose_save_directory_state: &mut ChooseSaveDirectoryState) -> Container<Message> {
    let dir_button_text = Text::new("Select Borderlands 3 Profile/Save File directory");
    let dir_button = Button::new(
        &mut choose_save_directory_state.choose_dir_button_state,
        Text::new("Select...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(JETBRAINS_MONO_BOLD)
            .size(18),
    )
    .on_press(InteractionMessage::ChooseSaveInteraction(
        ChooseSaveInteractionMessage::ChooseDirPressed,
    ))
    .padding(5)
    .into_interaction_element();

    let contents = Column::new()
        .push(dir_button_text)
        .push(dir_button)
        .spacing(20);

    Container::new(contents)
}
