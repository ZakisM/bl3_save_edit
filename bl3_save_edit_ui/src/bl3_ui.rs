use iced::{Application, Clipboard, Column, Command, Container, Element, Length};

use bl3_save_edit_core::bl3_save::util::{experience_to_level, REQUIRED_XP_LIST};

use crate::views::manage_save;
use crate::views::manage_save::character::{
    CharacterGearMessage, CharacterMessage, CharacterSduMessage, CharacterSkinMessage,
};
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
                    CharacterMessage::NameInputChanged(name_input) => {
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
                    CharacterMessage::SkinMessage(skin_message) => match skin_message {
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
                    CharacterMessage::GearMessage(gear_message) => match gear_message {
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
                    CharacterMessage::SduMessage(sdu_message) => match sdu_message {
                        CharacterSduMessage::BackpackInputChanged(level) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .backpack_input = level;
                        }
                        CharacterSduMessage::SniperInputChanged(level) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .sniper_input = level;
                        }
                        CharacterSduMessage::ShotgunInputChanged(level) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .shotgun_input = level;
                        }
                        CharacterSduMessage::PistolInputChanged(level) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .pistol_input = level;
                        }
                        CharacterSduMessage::GrenadeInputChanged(level) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .grenade_input = level;
                        }
                        CharacterSduMessage::SmgInputChanged(level) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .smg_input = level;
                        }
                        CharacterSduMessage::AssaultRifleInputChanged(level) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .assault_rifle_input = level;
                        }
                        CharacterSduMessage::HeavyInputChanged(level) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .heavy_input = level;
                        }
                    },
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
