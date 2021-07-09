use iced::{
    Align, Application, Clipboard, Color, Column, Command, Container, Element, HorizontalAlignment,
    Length, Row, Text,
};

use bl3_save_edit_core::bl3_save::util::{experience_to_level, REQUIRED_XP_LIST};

use crate::bl3_ui_style::{Bl3UiContentStyle, Bl3UiMenuBarStyle};
use crate::interaction;
use crate::resources::fonts::COMPACTA;
use crate::views::choose_save_directory::{
    ChooseSaveDirectoryState, ChooseSaveInteractionMessage, ChooseSaveMessage,
};
use crate::views::manage_save::character::{
    CharacterGearMessage, CharacterInteractionMessage, CharacterInteractionSduMessage,
    CharacterMessage, CharacterSkinMessage,
};
use crate::views::manage_save::general::GeneralInteractionMessage;
use crate::views::manage_save::main::{MainInteractionMessage, MainTabBarView};
use crate::views::manage_save::{
    ManageSaveInteractionMessage, ManageSaveMessage, ManageSaveState, ManageSaveView,
};
use crate::views::{choose_save_directory, manage_save};

#[derive(Debug)]
pub struct Bl3Ui {
    view_state: ViewState,
    choose_save_directory_state: ChooseSaveDirectoryState,
    manage_save_state: ManageSaveState,
}

#[derive(Debug)]
pub enum Message {
    InteractionMessage(InteractionMessage),
    ChooseSave(ChooseSaveMessage),
    ManageSave(ManageSaveMessage),
}

#[derive(Debug, Clone)]
pub enum InteractionMessage {
    ChooseSaveInteraction(ChooseSaveInteractionMessage),
    ManageSaveInteraction(ManageSaveInteractionMessage),
    Ignore,
}

#[derive(Debug, PartialEq)]
enum ViewState {
    ChooseSaveDirectory,
    ManageSave(ManageSaveView),
}

impl Application for Bl3Ui {
    type Executor = tokio::runtime::Runtime;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Bl3Ui {
                view_state: ViewState::ChooseSaveDirectory,
                choose_save_directory_state: ChooseSaveDirectoryState::default(),
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
            Message::InteractionMessage(interaction_msg) => match interaction_msg {
                InteractionMessage::ChooseSaveInteraction(choose_save_msg) => {
                    return match choose_save_msg {
                        ChooseSaveInteractionMessage::ChooseDirPressed => {
                            self.choose_save_directory_state.choose_dir_window_open = true;

                            Command::perform(interaction::choose_save_directory::choose(), |r| {
                                Message::ChooseSave(ChooseSaveMessage::ChooseDirCompleted(r))
                            })
                        }
                    };
                }
                InteractionMessage::ManageSaveInteraction(manage_save_msg) => match manage_save_msg
                {
                    ManageSaveInteractionMessage::Main(main_msg) => match main_msg {
                        MainInteractionMessage::TabBarGeneralPressed => {
                            self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                                MainTabBarView::General,
                            ))
                        }
                        MainInteractionMessage::TabBarCharacterPressed => {
                            self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                                MainTabBarView::Character,
                            ))
                        }
                        MainInteractionMessage::TabBarVehiclePressed => {
                            self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                                MainTabBarView::Vehicle,
                            ))
                        }
                        MainInteractionMessage::TabBarCurrencyPressed => {
                            self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                                MainTabBarView::Currency,
                            ))
                        }
                        MainInteractionMessage::TabBarFastTravelPressed => {
                            self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                                MainTabBarView::FastTravel,
                            ))
                        }
                    },
                    ManageSaveInteractionMessage::General(general_msg) => match general_msg {
                        GeneralInteractionMessage::GuidInputChanged(guid_input) => {
                            self.manage_save_state.main_state.general_state.guid_input = guid_input;
                        }
                        GeneralInteractionMessage::SlotInputChanged(slot_input) => {
                            self.manage_save_state.main_state.general_state.slot_input = slot_input;
                        }
                    },
                    ManageSaveInteractionMessage::Character(character_msg) => match character_msg {
                        CharacterInteractionMessage::NameInputChanged(name_input) => {
                            self.manage_save_state.main_state.character_state.name_input =
                                name_input;
                        }
                        CharacterInteractionMessage::XpLevelInputChanged(level) => {
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
                        CharacterInteractionMessage::XpPointsInputChanged(xp) => {
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
                        CharacterInteractionMessage::SduMessage(sdu_message) => match sdu_message {
                            CharacterInteractionSduMessage::BackpackInputChanged(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .backpack_input = level;
                            }
                            CharacterInteractionSduMessage::SniperInputChanged(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .sniper_input = level;
                            }
                            CharacterInteractionSduMessage::ShotgunInputChanged(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .shotgun_input = level;
                            }
                            CharacterInteractionSduMessage::PistolInputChanged(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .pistol_input = level;
                            }
                            CharacterInteractionSduMessage::GrenadeInputChanged(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .grenade_input = level;
                            }
                            CharacterInteractionSduMessage::SmgInputChanged(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .smg_input = level;
                            }
                            CharacterInteractionSduMessage::AssaultRifleInputChanged(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .assault_rifle_input = level;
                            }
                            CharacterInteractionSduMessage::HeavyInputChanged(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .heavy_input = level;
                            }
                        },
                    },
                },
                InteractionMessage::Ignore => {}
            },
            Message::ChooseSave(choose_save_msg) => match choose_save_msg {
                ChooseSaveMessage::ChooseDirCompleted(dir) => {
                    self.choose_save_directory_state.choose_dir_window_open = false;
                    dbg!(&dir);
                }
            },
            Message::ManageSave(manage_save_msg) => match manage_save_msg {
                ManageSaveMessage::Character(character_msg) => match character_msg {
                    CharacterMessage::PlayerClassSelected(player_class) => {
                        self.manage_save_state
                            .main_state
                            .character_state
                            .player_class_selected_class = player_class;
                    }
                    CharacterMessage::SkinMessage(skin_msg) => match skin_msg {
                        CharacterSkinMessage::HeadSkinSelected(selected) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .skin_state
                                .head_skin_selected = selected;
                        }
                        CharacterSkinMessage::CharacterSkinSelected(selected) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .skin_state
                                .character_skin_selected = selected;
                        }
                        CharacterSkinMessage::EchoThemeSelected(selected) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .skin_state
                                .echo_theme_selected = selected;
                        }
                    },
                    CharacterMessage::GearMessage(gear_msg) => match gear_msg {
                        CharacterGearMessage::UnlockGrenadeSlot(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_grenade_slot = b;
                        }
                        CharacterGearMessage::UnlockShieldSlot(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_shield_slot = b;
                        }
                        CharacterGearMessage::UnlockWeapon1Slot(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_weapon_1_slot = b;
                        }
                        CharacterGearMessage::UnlockWeapon2Slot(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_weapon_2_slot = b;
                        }
                        CharacterGearMessage::UnlockWeapon3Slot(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_weapon_3_slot = b;
                        }
                        CharacterGearMessage::UnlockWeapon4Slot(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_weapon_4_slot = b;
                        }
                        CharacterGearMessage::UnlockArtifactSlot(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_artifact_slot = b;
                        }
                        CharacterGearMessage::UnlockClassModSlot(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_class_mod_slot = b;
                        }
                    },
                },
            },
        };

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let title = Text::new("Borderlands 3 Save Edit".to_uppercase())
            .font(COMPACTA)
            .size(48)
            .color(Color::from_rgb8(242, 203, 5))
            .width(Length::Fill)
            .horizontal_alignment(HorizontalAlignment::Left);

        let menu_bar = Container::new(
            Row::new()
                .push(title)
                .spacing(25)
                .align_items(Align::Center),
        )
        .padding(20)
        .width(Length::Fill)
        .style(Bl3UiMenuBarStyle);

        let content = match &self.view_state {
            ViewState::ChooseSaveDirectory => {
                choose_save_directory::view(&mut self.choose_save_directory_state)
            }
            ViewState::ManageSave(manage_save_view) => match manage_save_view {
                ManageSaveView::TabBar(main_tab_bar_view) => {
                    manage_save::main::view(&mut self.manage_save_state, main_tab_bar_view)
                }
            },
        };

        let all_content = Column::new().push(menu_bar).push(content);

        Container::new(all_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(Bl3UiContentStyle)
            .into()
    }
}
