use std::path::PathBuf;

use iced::alignment::{Horizontal, Vertical};
use iced::{button, Alignment, Button, Color, Column, Container, Length, Text};

use bl3_save_edit_core::file_helper::Bl3FileType;

use crate::bl3_ui::{Bl3Message, InteractionMessage, MessageResult};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::views::InteractionExt;

#[derive(Debug, Default)]
pub struct ChooseSaveDirectoryState {
    choose_dir_button_state: button::State,
    pub choose_dir_window_open: bool,
}

#[derive(Debug, Clone)]
pub enum ChooseSaveMessage {
    ChooseDirCompleted(MessageResult<PathBuf>),
    FilesLoaded(MessageResult<(PathBuf, Vec<Bl3FileType>)>),
}

#[derive(Debug, Clone)]
pub enum ChooseSaveInteractionMessage {
    ChooseDirPressed,
}

pub fn view(choose_save_directory_state: &mut ChooseSaveDirectoryState) -> Container<Bl3Message> {
    let dir_button_text = Text::new("Select Borderlands 3 Save/Profile folder")
        .font(JETBRAINS_MONO)
        .size(20)
        .color(Color::from_rgb8(220, 220, 220));

    let mut dir_button = Button::new(
        &mut choose_save_directory_state.choose_dir_button_state,
        Text::new("Select...")
            .horizontal_alignment(Horizontal::Center)
            .font(JETBRAINS_MONO)
            .size(18),
    )
    .padding(10)
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
        .align_items(Alignment::Center);

    Container::new(contents)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}
