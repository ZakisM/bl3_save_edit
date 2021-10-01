use std::path::PathBuf;

use iced::{
    button, text_input, Alignment, Button, Color, Column, Container, Length, Row, Text, TextInput,
};

use crate::bl3_ui::{Bl3Message, InteractionMessage, MessageResult};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;

#[derive(Debug, Default)]
pub struct SettingsState {
    pub config_dir_input: String,
    pub config_dir_input_state: text_input::State,
    pub open_config_dir_button_state: button::State,
    pub backup_dir_input: String,
    pub backup_dir_input_state: text_input::State,
    pub open_backup_dir_button_state: button::State,
    pub change_backup_dir_button_state: button::State,
    pub choose_backup_dir_window_open: bool,
    pub saves_dir_input: String,
    pub saves_dir_input_state: text_input::State,
    pub open_saves_dir_button_state: button::State,
    pub change_saves_dir_button_state: button::State,
    pub choose_saves_dir_window_open: bool,
    pub decrease_ui_scale_button_state: button::State,
    pub increase_ui_scale_button_state: button::State,
    pub ui_scale_factor: f64,
}

#[derive(Debug, Clone)]
pub enum SettingsInteractionMessage {
    OpenConfigDir,
    OpenConfigDirCompleted(MessageResult<()>),
    OpenBackupDir,
    OpenBackupDirCompleted(MessageResult<()>),
    ChangeBackupDir,
    ChangeBackupDirCompleted(MessageResult<PathBuf>),
    OpenSavesDir,
    OpenSavesDirCompleted(MessageResult<()>),
    ChangeSavesDir,
    ChangeSavesDirCompleted(MessageResult<PathBuf>),
    DecreaseUIScale,
    IncreaseUIScale,
}

pub fn view(settings_state: &mut SettingsState) -> Container<Bl3Message> {
    let mut change_backup_dir_button = Button::new(
        &mut settings_state.change_backup_dir_button_state,
        Text::new("Change Folder")
            .font(JETBRAINS_MONO_BOLD)
            .size(17),
    )
    .padding(10)
    .style(Bl3UiStyle);

    if !settings_state.choose_backup_dir_window_open {
        change_backup_dir_button = change_backup_dir_button.on_press(
            InteractionMessage::SettingsInteraction(SettingsInteractionMessage::ChangeBackupDir),
        );
    }

    let config_dir = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "Config folder",
                    Length::Units(140),
                    TextInput::new(
                        &mut settings_state.config_dir_input_state,
                        "Loading config...",
                        &settings_state.config_dir_input,
                        |_| InteractionMessage::Ignore,
                    )
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut settings_state.open_config_dir_button_state,
                    Text::new("Open Folder").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::SettingsInteraction(
                    SettingsInteractionMessage::OpenConfigDir,
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let backup_dir = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "Backups folder",
                    Length::Units(140),
                    TextInput::new(
                        &mut settings_state.backup_dir_input_state,
                        "Choose a saves folder first...",
                        &settings_state.backup_dir_input,
                        |_| InteractionMessage::Ignore,
                    )
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut settings_state.open_backup_dir_button_state,
                    Text::new("Open Folder").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::SettingsInteraction(
                    SettingsInteractionMessage::OpenBackupDir,
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .push(change_backup_dir_button.into_element())
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let mut change_saves_dir_button = Button::new(
        &mut settings_state.change_saves_dir_button_state,
        Text::new("Change Folder")
            .font(JETBRAINS_MONO_BOLD)
            .size(17),
    )
    .padding(10)
    .style(Bl3UiStyle);

    if !settings_state.choose_saves_dir_window_open {
        change_saves_dir_button = change_saves_dir_button.on_press(
            InteractionMessage::SettingsInteraction(SettingsInteractionMessage::ChangeSavesDir),
        );
    }

    let saves_dir = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "Saves folder",
                    Length::Units(140),
                    TextInput::new(
                        &mut settings_state.saves_dir_input_state,
                        "Choose a saves folder first...",
                        &settings_state.saves_dir_input,
                        |_| InteractionMessage::Ignore,
                    )
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut settings_state.open_saves_dir_button_state,
                    Text::new("Open Folder").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::SettingsInteraction(
                    SettingsInteractionMessage::OpenSavesDir,
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .push(change_saves_dir_button.into_element())
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let ui_scale = Container::new(
        LabelledElement::create(
            "UI Scale",
            Length::Units(140),
            Row::new()
                .push(
                    Button::new(
                        &mut settings_state.decrease_ui_scale_button_state,
                        Text::new("  -  ").font(JETBRAINS_MONO_BOLD).size(17),
                    )
                    .on_press(InteractionMessage::SettingsInteraction(
                        SettingsInteractionMessage::DecreaseUIScale,
                    ))
                    .padding(10)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .push(
                    Text::new(format!("{:.2}", settings_state.ui_scale_factor))
                        .color(Color::from_rgb8(220, 220, 220))
                        .font(JETBRAINS_MONO_BOLD)
                        .size(17),
                )
                .push(
                    Button::new(
                        &mut settings_state.increase_ui_scale_button_state,
                        Text::new("  +  ").font(JETBRAINS_MONO_BOLD).size(17),
                    )
                    .on_press(InteractionMessage::SettingsInteraction(
                        SettingsInteractionMessage::IncreaseUIScale,
                    ))
                    .padding(10)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(20)
                .align_items(Alignment::Center),
        )
        .spacing(15)
        .width(Length::Fill)
        .align_items(Alignment::Center),
    )
    .style(Bl3UiStyle);

    let all_contents = Column::new()
        .push(config_dir)
        .push(backup_dir)
        .push(saves_dir)
        .push(ui_scale)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
