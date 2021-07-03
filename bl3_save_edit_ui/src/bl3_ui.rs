use iced::{
    button, container, Align, Application, Clipboard, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Text,
};

use bl3_save_edit_core::bl3_save::util::{experience_to_level, REQUIRED_XP_LIST};

use crate::resources::fonts::COMPACTA;
use crate::views::manage_save;
use crate::views::manage_save::character::CharacterMessage;
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
                view_state: ViewState::ManageSave(ManageSaveView::TabBar(
                    MainTabBarView::Character,
                )),
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
                ManageSaveMessage::Character(character_msg) => match character_msg {
                    CharacterMessage::CharacterNameInputChanged(name_input) => {
                        self.manage_save_state.main_state.character_state.name_input = name_input;
                    }
                    CharacterMessage::PlayerClassSelected(selected) => {
                        self.manage_save_state
                            .main_state
                            .character_state
                            .player_class_selected_class = selected;
                    }
                    CharacterMessage::XpLevelInputChanged(level) => {
                        let xp_points = if level > 0 {
                            REQUIRED_XP_LIST[level - 1][0] as usize
                        } else {
                            0
                        };

                        self.manage_save_state
                            .main_state
                            .character_state
                            .xp_level_input = level;

                        self.manage_save_state
                            .main_state
                            .character_state
                            .xp_points_input = xp_points;
                    }
                    CharacterMessage::XpPointsInputChanged(xp) => {
                        let level = experience_to_level(xp as i32).unwrap_or(0) as usize;

                        self.manage_save_state
                            .main_state
                            .character_state
                            .xp_points_input = xp;

                        self.manage_save_state
                            .main_state
                            .character_state
                            .xp_level_input = level;
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
            .into()
    }
}
