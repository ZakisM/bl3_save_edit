use iced::{
    button, Align, Application, Clipboard, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Text,
};

use crate::bl3_ui_style::{Bl3UiStyle, PRIMARY_COLOR};
use crate::fonts::COMPACTA;
use crate::views::manage_save::main::{MainMessage, MainTabBarView};
use crate::views::manage_save::{ManageSaveMessage, ManageSaveState, ManageSaveView};
use crate::views::save_selection_from_dir::{SaveSelectionMessage, SaveSelectionState};
use crate::views::{manage_save, save_selection_from_dir};

#[derive(Debug)]
pub struct Bl3Ui {
    view_state: ViewState,
    save_selection_state: SaveSelectionState,
    manage_save_state: ManageSaveState,
}

#[derive(Debug, Clone)]
pub enum Message {
    SaveSelection(SaveSelectionMessage),
    ManageSave(ManageSaveMessage),
}

#[derive(Debug, PartialEq)]
enum ViewState {
    SaveSelectionFromDir,
    ManageSave(ManageSaveView),
}

impl Application for Bl3Ui {
    type Executor = tokio::runtime::Runtime;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Bl3Ui {
                view_state: ViewState::ManageSave(ManageSaveView::TabBar(MainTabBarView::General)),
                save_selection_state: SaveSelectionState::default(),
                manage_save_state: ManageSaveState::default(),
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
            Message::SaveSelection(msg) => match msg {
                SaveSelectionMessage::SavePressed => {
                    self.view_state =
                        ViewState::ManageSave(ManageSaveView::TabBar(MainTabBarView::General))
                }
            },
            Message::ManageSave(manage_save_msg) => match manage_save_msg {
                ManageSaveMessage::Main(main_msg) => match main_msg {
                    MainMessage::TabBarGeneralPressed => {
                        self.view_state =
                            ViewState::ManageSave(ManageSaveView::TabBar(MainTabBarView::General))
                    }
                    MainMessage::TabBarCharacterPressed => {
                        self.view_state =
                            ViewState::ManageSave(ManageSaveView::TabBar(MainTabBarView::Character))
                    }
                },
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

        let content = match &self.view_state {
            ViewState::SaveSelectionFromDir => save_selection_from_dir::view(
                &mut self.save_selection_state.save_selection_button_state,
            ),
            ViewState::ManageSave(manage_save_view) => match manage_save_view {
                ManageSaveView::TabBar(main_tab_bar_view) => match main_tab_bar_view {
                    MainTabBarView::General => manage_save::main::view(
                        &mut self.manage_save_state,
                        MainTabBarView::General,
                    ),
                    MainTabBarView::Character => manage_save::main::view(
                        &mut self.manage_save_state,
                        MainTabBarView::Character,
                    ),
                },
            },
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
