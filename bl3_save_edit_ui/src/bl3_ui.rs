use iced::{
    button, Align, Application, Clipboard, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Text,
};

use crate::bl3_ui_style::{Bl3UiStyle, PRIMARY_COLOR};
use crate::fonts::COMPACTA;
use crate::views::{manage_save, save_selection_from_dir};

#[derive(Debug)]
pub struct Bl3Ui {
    view_state: ViewState,
    save_selection_state: SaveSelectionState,
}

#[derive(Debug, Clone)]
pub enum Message {
    SaveSelectionMessage(SaveSelectionMessage),
}

#[derive(Debug, Clone)]
pub enum SaveSelectionMessage {
    SavePressed,
}

#[derive(Debug, Default)]
struct SaveSelectionState {
    save_selection_button_state: button::State,
}

#[derive(Debug, PartialEq)]
enum ViewState {
    SaveSelectionFromDir,
    ManageSaveMain,
}

impl Application for Bl3Ui {
    type Executor = tokio::runtime::Runtime;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Bl3Ui {
                view_state: ViewState::ManageSaveMain,
                save_selection_state: SaveSelectionState::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Borderlands 3 Save Edit")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::SaveSelectionMessage(msg) => match msg {
                SaveSelectionMessage::SavePressed => self.view_state = ViewState::ManageSaveMain,
            },
        };

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let title = Text::new("Borderlands 3 Save Editor".to_uppercase())
            .font(COMPACTA)
            .size(50)
            .color(Color::from_rgb8(
                PRIMARY_COLOR.0,
                PRIMARY_COLOR.1,
                PRIMARY_COLOR.2,
            ))
            .width(Length::Fill)
            .horizontal_alignment(HorizontalAlignment::Left);

        // let title_bar = Container::new(
        //     Row::new()
        //         .push(title)
        //         .spacing(25)
        //         .align_items(Align::Center),
        // )
        // .padding(20)
        // .width(Length::Fill)
        // .style(MenuRowStyle);

        let content = match self.view_state {
            ViewState::SaveSelectionFromDir => save_selection_from_dir::view(
                &mut self.save_selection_state.save_selection_button_state,
            ),
            ViewState::ManageSaveMain => manage_save::main::view(),
        };

        // let all_content = Column::new().push(title_bar).push(content);
        let all_content = Column::new().push(content);

        Container::new(all_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(Bl3UiStyle)
            .into()
    }
}
