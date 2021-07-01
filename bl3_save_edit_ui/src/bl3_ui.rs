use iced::{
    button, Align, Application, Clipboard, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Text,
};

use crate::bl3_ui_style::{Bl3UiStyle, PRIMARY_COLOR};
use crate::resources::fonts::COMPACTA;
use crate::views::manage_save;
use crate::views::manage_save::general::GeneralMessage;
use crate::views::manage_save::main::{MainMessage, MainTabBarView};
use crate::views::manage_save::{ManageSaveMessage, ManageSaveState, ManageSaveView};

#[derive(Debug)]
pub struct Bl3Ui {
    view_state: ViewState,
    manage_save_state: ManageSaveState,
}

#[derive(Debug, Clone)]
pub enum Message {
    ManageSave(ManageSaveMessage),
    Ignore,
}

#[derive(Debug, PartialEq)]
enum ViewState {
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
                    MainMessage::TabBarVehiclePressed => {
                        self.view_state =
                            ViewState::ManageSave(ManageSaveView::TabBar(MainTabBarView::Vehicle))
                    }
                    MainMessage::TabBarCurrencyPressed => {
                        self.view_state =
                            ViewState::ManageSave(ManageSaveView::TabBar(MainTabBarView::Currency))
                    }
                    MainMessage::TabBarFastTravelPressed => {
                        self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                            MainTabBarView::FastTravel,
                        ))
                    }
                },
                ManageSaveMessage::General(general_msg) => match general_msg {
                    GeneralMessage::GuidInputChanged(guid_input) => {
                        self.manage_save_state.main_state.general_state.guid_input = guid_input;
                    }
                    GeneralMessage::SlotInputChanged(slot_input) => {
                        self.manage_save_state.main_state.general_state.slot_input = slot_input;
                    }
                },
            },
            Message::Ignore => (),
        };

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = match &self.view_state {
            ViewState::ManageSave(manage_save_view) => match manage_save_view {
                ManageSaveView::TabBar(main_tab_bar_view) => {
                    manage_save::main::view(&mut self.manage_save_state, main_tab_bar_view)
                }
            },
        };

        let all_content = Column::new().push(content);

        Container::new(all_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(Bl3UiStyle)
            .into()
    }
}
