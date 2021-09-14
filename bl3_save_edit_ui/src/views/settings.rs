use iced::{
    button, text_input, Align, Button, Color, Column, Container, Length, Row, Text, TextInput,
};

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::choose_save_directory::ChooseSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;

#[derive(Debug, Default)]
pub struct SettingsState {
    pub backup_folder_input: String,
    pub backup_folder_input_state: text_input::State,
    pub open_backup_folder_button_state: button::State,
    pub saves_folder_input: String,
    pub saves_folder_input_state: text_input::State,
    pub change_saves_folder_button_state: button::State,
    pub decrease_ui_scale_button_state: button::State,
    pub increase_ui_scale_button_state: button::State,
    pub ui_scale_factor: f64,
}

#[derive(Debug, Clone)]
pub enum SettingsInteractionMessage {
    OpenBackupFolder,
    DecreaseUIScale,
    IncreaseUIScale,
}

pub fn view(
    settings_state: &mut SettingsState,
    choose_dir_window_open: bool,
) -> Container<Bl3Message> {
    let backup_folder = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "Backup folder",
                    Length::Units(140),
                    TextInput::new(
                        &mut settings_state.backup_folder_input_state,
                        "Choose a saves folder first...",
                        &settings_state.backup_folder_input,
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
                .align_items(Align::Center),
            )
            .push(
                Button::new(
                    &mut settings_state.open_backup_folder_button_state,
                    Text::new("Open Backups Folder")
                        .font(JETBRAINS_MONO_BOLD)
                        .size(17),
                )
                .on_press(InteractionMessage::SettingsInteraction(
                    SettingsInteractionMessage::OpenBackupFolder,
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Align::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let mut change_saves_folder_button = Button::new(
        &mut settings_state.change_saves_folder_button_state,
        Text::new("Change Saves Folder")
            .font(JETBRAINS_MONO_BOLD)
            .size(17),
    )
    .padding(10)
    .style(Bl3UiStyle);

    if !choose_dir_window_open {
        change_saves_folder_button =
            change_saves_folder_button.on_press(InteractionMessage::ChooseSaveInteraction(
                ChooseSaveInteractionMessage::ChooseDirPressed,
            ));
    }

    let saves_folder = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "Saves folder",
                    Length::Units(140),
                    TextInput::new(
                        &mut settings_state.saves_folder_input_state,
                        "Choose a saves folder first...",
                        &settings_state.saves_folder_input,
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
                .align_items(Align::Center),
            )
            .push(change_saves_folder_button.into_element())
            .align_items(Align::Center),
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
                .align_items(Align::Center),
        )
        .spacing(15)
        .width(Length::Fill)
        .align_items(Align::Center),
    )
    .style(Bl3UiStyle);

    let all_contents = Column::new()
        .push(backup_folder)
        .push(saves_folder)
        .push(ui_scale)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
