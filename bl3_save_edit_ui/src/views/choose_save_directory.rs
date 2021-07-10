use std::path::PathBuf;

use anyhow::Result;
use iced::{button, Align, Button, Color, Column, Container, HorizontalAlignment, Length, Text};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::interaction::InteractionExt;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};

#[derive(Debug, Default)]
pub struct ChooseSaveDirectoryState {
    choose_dir_button_state: button::State,
    pub choose_dir_window_open: bool,
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
    let dir_button_text = Text::new("Select Borderlands 3 Save/Profile directory")
        .font(JETBRAINS_MONO)
        .size(20)
        .color(Color::from_rgb8(220, 220, 220));

    let mut dir_button = Button::new(
        &mut choose_save_directory_state.choose_dir_button_state,
        Text::new("Select...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(JETBRAINS_MONO_BOLD)
            .size(18),
    )
    .padding(5)
    .style(Bl3UiStyle);

    if !choose_save_directory_state.choose_dir_window_open {
        dir_button = dir_button.on_press(InteractionMessage::ChooseSaveInteraction(
            ChooseSaveInteractionMessage::ChooseDirPressed,
        ));
    }

    let contents = Column::new()
        .push(dir_button_text)
        .push(dir_button.into_element())
        .spacing(20)
        .align_items(Align::Center);

    Container::new(contents)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Align::Center)
        .align_y(Align::Center)
}
