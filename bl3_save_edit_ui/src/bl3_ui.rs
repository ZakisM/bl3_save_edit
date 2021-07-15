use iced::{
    button, Align, Application, Button, Clipboard, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Text,
};

use bl3_save_edit_core::bl3_save::sdu::SaveSduSlot;
use bl3_save_edit_core::bl3_save::util::{experience_to_level, REQUIRED_XP_LIST};
use bl3_save_edit_core::file_helper::Bl3FileType;

use crate::bl3_ui_style::{Bl3UiContentStyle, Bl3UiMenuBarStyle, Bl3UiStyle};
use crate::interaction;
use crate::interaction::InteractionExt;
use crate::resources::fonts::{COMPACTA, JETBRAINS_MONO_BOLD};
use crate::state_mappers;
use crate::views::choose_save_directory::{
    ChooseSaveDirectoryState, ChooseSaveInteractionMessage, ChooseSaveMessage,
};
use crate::views::manage_save::character::{
    CharacterGearMessage, CharacterInteractionMessage, CharacterInteractionSduMessage,
    CharacterMessage, CharacterSkinMessage,
};
use crate::views::manage_save::fast_travel::{FastTravelInteractionMessage, FastTravelMessage};
use crate::views::manage_save::general::{GeneralInteractionMessage, GeneralMessage};
use crate::views::manage_save::main::{MainInteractionMessage, MainTabBarView};
use crate::views::manage_save::{
    ManageSaveInteractionMessage, ManageSaveMessage, ManageSaveState, ManageSaveView,
};
use crate::views::{choose_save_directory, manage_save};

#[derive(Debug, Default)]
pub struct Bl3Ui {
    view_state: ViewState,
    choose_save_directory_state: ChooseSaveDirectoryState,
    manage_save_state: ManageSaveState,
    loaded_files: Vec<Bl3FileType>,
    save_file_button_state: button::State,
}

#[derive(Debug)]
pub enum Message {
    InteractionMessage(InteractionMessage),
    ChooseSave(ChooseSaveMessage),
    ManageSave(ManageSaveMessage),
    SaveFileCompleted(std::result::Result<(), std::io::Error>),
}

#[derive(Debug, Clone)]
pub enum InteractionMessage {
    ChooseSaveInteraction(ChooseSaveInteractionMessage),
    ManageSaveInteraction(ManageSaveInteractionMessage),
    SaveFilePressed,
    Ignore,
}

#[derive(Debug, PartialEq)]
enum ViewState {
    ChooseSaveDirectory,
    ManageSave(ManageSaveView),
}

impl std::default::Default for ViewState {
    fn default() -> Self {
        Self::ChooseSaveDirectory
    }
}

impl Application for Bl3Ui {
    type Executor = tokio::runtime::Runtime;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Bl3Ui {
                view_state: ViewState::ChooseSaveDirectory,
                ..Bl3Ui::default()
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
                        GeneralInteractionMessage::GenerateGuidPressed => {
                            return Command::perform(
                                interaction::manage_save::general::generate_random_guid(),
                                |r| {
                                    Message::ManageSave(ManageSaveMessage::General(
                                        GeneralMessage::GenerateRandomGuidCompleted(r),
                                    ))
                                },
                            );
                        }
                    },
                    ManageSaveInteractionMessage::Character(character_msg) => match character_msg {
                        CharacterInteractionMessage::NameInputChanged(name_input) => {
                            self.manage_save_state.main_state.character_state.name_input =
                                name_input;
                        }
                        CharacterInteractionMessage::XpLevelInputChanged(level) => {
                            let xp_points = if level > 0 {
                                REQUIRED_XP_LIST[level as usize - 1][0]
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
                            let level = experience_to_level(xp as i32).unwrap_or(0);

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
                        CharacterInteractionMessage::MaxSduSlotsPressed => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .backpack_input = SaveSduSlot::Backpack.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .sniper_input = SaveSduSlot::Sniper.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .shotgun_input = SaveSduSlot::Shotgun.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .pistol_input = SaveSduSlot::Pistol.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .grenade_input = SaveSduSlot::Grenade.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .smg_input = SaveSduSlot::Smg.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .assault_rifle_input = SaveSduSlot::Ar.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_state
                                .heavy_input = SaveSduSlot::Heavy.maximum();
                        }
                    },
                    ManageSaveInteractionMessage::FastTravel(fast_travel_msg) => {
                        match fast_travel_msg {
                            FastTravelInteractionMessage::UncheckAllVisitedTeleporterList => {
                                self.manage_save_state
                                    .main_state
                                    .fast_travel_state
                                    .visited_teleporters_list
                                    .iter_mut()
                                    .for_each(|vt| vt.visited = false);
                            }
                            FastTravelInteractionMessage::CheckAllVisitedTeleporterList => {
                                self.manage_save_state
                                    .main_state
                                    .fast_travel_state
                                    .visited_teleporters_list
                                    .iter_mut()
                                    .for_each(|vt| vt.visited = true);
                            }
                        }
                    }
                },
                InteractionMessage::SaveFilePressed => {
                    let current_file = &mut self.manage_save_state.current_file;

                    current_file.character_data.set_head_skin_selected(
                        &self
                            .manage_save_state
                            .main_state
                            .character_state
                            .skin_state
                            .head_skin_selected,
                    );

                    // current_file.character_data.set_active_travel_stations(
                    //     self.manage_save_state
                    //         .main_state
                    //         .fast_travel_state
                    //         .playthrough_type_selected as usize,
                    //     &self
                    //         .manage_save_state
                    //         .main_state
                    //         .fast_travel_state
                    //         .visited_teleporters_list,
                    // );

                    let output_file = self
                        .choose_save_directory_state
                        .saves_dir
                        .join("test_file_zak.sav");

                    match current_file.to_bytes() {
                        Ok(output) => {
                            return Command::perform(
                                tokio::fs::write(output_file, output),
                                Message::SaveFileCompleted,
                            );
                        }
                        Err(e) => eprintln!("{}", e),
                    };
                }
                InteractionMessage::Ignore => {}
            },
            Message::ChooseSave(choose_save_msg) => match choose_save_msg {
                ChooseSaveMessage::ChooseDirCompleted(dir) => {
                    self.choose_save_directory_state.choose_dir_window_open = false;
                    match dir {
                        Ok(dir) => {
                            self.choose_save_directory_state.saves_dir = dir.clone();

                            return Command::perform(
                                interaction::choose_save_directory::load_files_in_directory(dir),
                                |files_loaded| {
                                    Message::ChooseSave(ChooseSaveMessage::LoadedFiles(
                                        files_loaded,
                                    ))
                                },
                            );
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                }
                ChooseSaveMessage::LoadedFiles(loaded_files) => match loaded_files {
                    Ok(files) => {
                        self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                            MainTabBarView::Character,
                        ));

                        self.loaded_files = files;

                        let first_file = self
                            .loaded_files
                            .get(0)
                            .expect("loaded_files list was empty");

                        match first_file {
                            Bl3FileType::PcSave(save) | Bl3FileType::Ps4Save(save) => {
                                self.manage_save_state.current_file = save.to_owned();

                                state_mappers::manage_save::general::map_general_state(
                                    &save,
                                    &mut self.manage_save_state,
                                );

                                state_mappers::manage_save::character::map_character_state(
                                    &save,
                                    &mut self.manage_save_state,
                                );

                                state_mappers::manage_save::fast_travel::map_fast_travel_state(
                                    &save,
                                    &mut self.manage_save_state,
                                );
                            }
                            Bl3FileType::PcProfile(p) | Bl3FileType::Ps4Profile(p) => (),
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                },
            },
            Message::ManageSave(manage_save_msg) => match manage_save_msg {
                ManageSaveMessage::General(general_msg) => match general_msg {
                    GeneralMessage::GenerateRandomGuidCompleted(guid) => {
                        self.manage_save_state.main_state.general_state.guid_input = guid;
                    }
                },
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
                ManageSaveMessage::FastTravel(fast_travel_msg) => match fast_travel_msg {
                    FastTravelMessage::LastVisitedTeleporterSelected(last_visited) => {
                        self.manage_save_state
                            .main_state
                            .fast_travel_state
                            .last_visited_teleporter_selected = last_visited;
                    }
                    FastTravelMessage::VisitedTeleportersListUpdated((index, visited)) => {
                        self.manage_save_state
                            .main_state
                            .fast_travel_state
                            .visited_teleporters_list
                            .get_mut(index)
                            .expect(
                                "failed to find fast travel station to update in teleporters list",
                            )
                            .visited = visited;
                    }
                    FastTravelMessage::PlaythroughSelected(playthrough_type) => {
                        self.manage_save_state
                            .main_state
                            .fast_travel_state
                            .playthrough_type_selected = playthrough_type;

                        let playthrough_id = playthrough_type as usize;

                        if let Some(playthrough) = self
                            .manage_save_state
                            .current_file
                            .character_data
                            .playthroughs
                            .get(playthrough_id)
                        {
                            self.manage_save_state
                                .main_state
                                .fast_travel_state
                                .visited_teleporters_list
                                .iter_mut()
                                .for_each(|vt| {
                                    vt.visited = false;

                                    if playthrough
                                        .active_travel_stations
                                        .iter()
                                        .any(|ats| ats.to_lowercase() == vt.game_data.ident)
                                    {
                                        vt.visited = true;
                                    }
                                });
                        }
                    }
                },
            },
            Message::SaveFileCompleted(result) => match result {
                Ok(_) => println!("Successfully saved file"),
                Err(e) => eprintln!("{}", e),
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

        let save_button = Button::new(
            &mut self.save_file_button_state,
            Text::new("Save").font(JETBRAINS_MONO_BOLD).size(17),
        )
        .on_press(InteractionMessage::SaveFilePressed)
        .padding(10)
        .style(Bl3UiStyle)
        .into_element();

        let mut menu_bar_content = Row::new()
            .push(title)
            .spacing(25)
            .align_items(Align::Center);

        if self.view_state != ViewState::ChooseSaveDirectory {
            menu_bar_content = menu_bar_content.push(save_button);
        }

        let menu_bar = Container::new(menu_bar_content)
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
