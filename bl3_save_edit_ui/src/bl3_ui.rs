use std::mem;

use iced::{
    button, pick_list, Align, Application, Button, Clipboard, Color, Column, Command, Container,
    Element, HorizontalAlignment, Length, PickList, Row, Text,
};

use bl3_save_edit_core::bl3_save::sdu::SaveSduSlot;
use bl3_save_edit_core::bl3_save::util::{experience_to_level, REQUIRED_XP_LIST};
use bl3_save_edit_core::file_helper::Bl3FileType;

use crate::bl3_ui_style::{Bl3UiContentStyle, Bl3UiMenuBarStyle, Bl3UiStyle};
use crate::commands::{initialization, interaction};
use crate::resources::fonts::{COMPACTA, JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::state_mappers::manage_save::fast_travel::map_fast_travel_stations_to_visited_teleporters_list;
use crate::state_mappers::manage_save::inventory::map_save_to_inventory_state;
use crate::views::choose_save_directory::{
    ChooseSaveDirectoryState, ChooseSaveInteractionMessage, ChooseSaveMessage,
};
use crate::views::initialization::InitializationMessage;
use crate::views::manage_save::character::{
    CharacterInteractionMessage, CharacterMessage, CharacterSduInputChangedMessage,
    CharacterSkinSelectedMessage, CharacterUnlockGearMessage,
};
use crate::views::manage_save::currency::CurrencyInteractionMessage;
use crate::views::manage_save::fast_travel::{FastTravelInteractionMessage, FastTravelMessage};
use crate::views::manage_save::general::{GeneralInteractionMessage, GeneralMessage};
use crate::views::manage_save::inventory::InventoryInteractionMessage;
use crate::views::manage_save::main::{MainTabBarInteractionMessage, MainTabBarView};
use crate::views::manage_save::{
    ManageSaveInteractionMessage, ManageSaveMessage, ManageSaveState, ManageSaveView,
};
use crate::views::InteractionExt;
use crate::{state_mappers, views, VERSION};

#[derive(Debug, Default)]
pub struct Bl3UiState {
    pub view_state: ViewState,
    choose_save_directory_state: ChooseSaveDirectoryState,
    pub manage_save_state: ManageSaveState,
    loaded_files_selector: pick_list::State<Bl3FileType>,
    pub loaded_files_selected: Box<Bl3FileType>,
    loaded_files: Vec<Bl3FileType>,
    change_dir_button_state: button::State,
    save_file_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Initialization(InitializationMessage),
    Interaction(InteractionMessage),
    ChooseSave(ChooseSaveMessage),
    ManageSave(ManageSaveMessage),
    SaveFileCompleted(MessageResult<()>),
}

#[derive(Debug, Clone)]
pub enum MessageResult<T> {
    Success(T),
    Error(String),
}

impl<T> MessageResult<T> {
    pub fn handle_result(result: anyhow::Result<T>) -> MessageResult<T> {
        match result {
            Ok(v) => MessageResult::Success(v),
            Err(e) => MessageResult::Error(e.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum InteractionMessage {
    ChooseSaveInteraction(ChooseSaveInteractionMessage),
    ManageSaveInteraction(ManageSaveInteractionMessage),
    LoadedFileSelected(Box<Bl3FileType>),
    Ignore,
}

#[derive(Debug, PartialEq)]
pub enum ViewState {
    Initializing,
    ChooseSaveDirectory,
    ManageSave(ManageSaveView),
}

impl std::default::Default for ViewState {
    fn default() -> Self {
        Self::ChooseSaveDirectory
    }
}

impl Application for Bl3UiState {
    type Executor = tokio::runtime::Runtime;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let initialization_tasks = [Command::perform(initialization::load_lazy_data(), |_| {
            Message::Initialization(InitializationMessage::LoadLazyData)
        })];

        (
            Bl3UiState {
                view_state: ViewState::Initializing,
                ..Bl3UiState::default()
            },
            Command::batch(initialization_tasks),
        )
    }

    fn title(&self) -> String {
        format!("Borderlands 3 Save Editor - v{}", VERSION)
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::Initialization(initialization_msg) => match initialization_msg {
                InitializationMessage::LoadLazyData => {
                    self.view_state = ViewState::ChooseSaveDirectory;
                }
            },
            Message::Interaction(interaction_msg) => match interaction_msg {
                InteractionMessage::ChooseSaveInteraction(choose_save_msg) => {
                    return match choose_save_msg {
                        ChooseSaveInteractionMessage::ChooseDirPressed => {
                            self.choose_save_directory_state.choose_dir_window_open = true;

                            Command::perform(interaction::choose_save_directory::choose(), |r| {
                                Message::ChooseSave(ChooseSaveMessage::ChooseDirCompleted(
                                    MessageResult::handle_result(r),
                                ))
                            })
                        }
                    };
                }
                InteractionMessage::ManageSaveInteraction(manage_save_msg) => match manage_save_msg
                {
                    ManageSaveInteractionMessage::Main(main_msg) => match main_msg {
                        MainTabBarInteractionMessage::General => {
                            self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                                MainTabBarView::General,
                            ))
                        }
                        MainTabBarInteractionMessage::Character => {
                            self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                                MainTabBarView::Character,
                            ))
                        }
                        MainTabBarInteractionMessage::Inventory => {
                            self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                                MainTabBarView::Inventory,
                            ))
                        }
                        MainTabBarInteractionMessage::Currency => {
                            self.view_state = ViewState::ManageSave(ManageSaveView::TabBar(
                                MainTabBarView::Currency,
                            ))
                        }
                        MainTabBarInteractionMessage::FastTravel => {
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
                        GeneralInteractionMessage::SaveTypeSelected(save_type) => {
                            self.manage_save_state
                                .main_state
                                .general_state
                                .save_type_selected = save_type;
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
                            let level = experience_to_level(xp as i32).unwrap_or(1);

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
                            CharacterSduInputChangedMessage::Backpack(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .backpack_input = level;
                            }
                            CharacterSduInputChangedMessage::Sniper(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .sniper_input = level;
                            }
                            CharacterSduInputChangedMessage::Shotgun(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .shotgun_input = level;
                            }
                            CharacterSduInputChangedMessage::Pistol(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .pistol_input = level;
                            }
                            CharacterSduInputChangedMessage::Grenade(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .grenade_input = level;
                            }
                            CharacterSduInputChangedMessage::Smg(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .smg_input = level;
                            }
                            CharacterSduInputChangedMessage::AssaultRifle(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_state
                                    .assault_rifle_input = level;
                            }
                            CharacterSduInputChangedMessage::Heavy(level) => {
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
                        CharacterInteractionMessage::PlayerClassSelected(player_class) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .player_class_selected_class = player_class;
                        }
                        CharacterInteractionMessage::SkinMessage(skin_message) => {
                            match skin_message {
                                CharacterSkinSelectedMessage::HeadSkin(selected) => {
                                    self.manage_save_state
                                        .main_state
                                        .character_state
                                        .skin_state
                                        .head_skin_selected = selected;
                                }
                                CharacterSkinSelectedMessage::CharacterSkin(selected) => {
                                    self.manage_save_state
                                        .main_state
                                        .character_state
                                        .skin_state
                                        .character_skin_selected = selected;
                                }
                                CharacterSkinSelectedMessage::EchoTheme(selected) => {
                                    self.manage_save_state
                                        .main_state
                                        .character_state
                                        .skin_state
                                        .echo_theme_selected = selected;
                                }
                            }
                        }
                    },
                    ManageSaveInteractionMessage::Currency(currency_msg) => match currency_msg {
                        CurrencyInteractionMessage::MoneyInputChanged(money) => {
                            self.manage_save_state.main_state.currency_state.money_input = money;
                        }
                        CurrencyInteractionMessage::EridiumInputChanged(eridium) => {
                            self.manage_save_state
                                .main_state
                                .currency_state
                                .eridium_input = eridium;
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
                            FastTravelInteractionMessage::LastVisitedTeleporterSelected(
                                last_visited,
                            ) => {
                                self.manage_save_state
                                    .main_state
                                    .fast_travel_state
                                    .last_visited_teleporter_selected = last_visited;
                            }
                            FastTravelInteractionMessage::PlaythroughSelected(playthrough_type) => {
                                self.manage_save_state
                                    .main_state
                                    .fast_travel_state
                                    .playthrough_type_selected = playthrough_type;

                                let playthrough_id = playthrough_type as usize;

                                map_fast_travel_stations_to_visited_teleporters_list(
                                    playthrough_id,
                                    &mut self.manage_save_state,
                                );
                            }
                        }
                    }
                    ManageSaveInteractionMessage::SaveFilePressed => {
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
                                    interaction::save_file::save_file(output_file, output),
                                    |r| Message::SaveFileCompleted(MessageResult::handle_result(r)),
                                );
                            }
                            Err(e) => eprintln!("{}", e),
                        };
                    }
                    ManageSaveInteractionMessage::Inventory(inventory_msg) => match inventory_msg {
                        InventoryInteractionMessage::ItemPressed(item_index) => {
                            // self.manage_save_state
                            //     .main_state
                            //     .inventory_state
                            //     .items
                            //     .iter_mut()
                            //     .enumerate()
                            //     .for_each(|(i, item)| {
                            //         item.is_active = false;
                            //
                            //         if item_index == i {
                            //             item.is_active = true;
                            //         }
                            //     });
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .selected_item_index = item_index;

                            map_save_to_inventory_state(&mut self.manage_save_state);
                        }
                        InventoryInteractionMessage::BalanceInputChanged(balance_input) => {
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .balance_input = balance_input;
                        }
                        InventoryInteractionMessage::BalanceInputSelected(balance_selected) => {
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .balance_input = balance_selected.ident.to_owned();

                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .balance_input_selected = balance_selected;
                        }
                        InventoryInteractionMessage::InventoryDataInputChanged(
                            inventory_data_input,
                        ) => {
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .inventory_data_input = inventory_data_input;
                        }
                        InventoryInteractionMessage::ManufacturerInputChanged(
                            manufacturer_input,
                        ) => {
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .manufacturer_input = manufacturer_input;
                        }
                    },
                },
                InteractionMessage::LoadedFileSelected(loaded_file) => {
                    self.loaded_files_selected = loaded_file;

                    state_mappers::map_loaded_file_to_state(self);
                }
                InteractionMessage::Ignore => {}
            },
            Message::ChooseSave(choose_save_msg) => match choose_save_msg {
                ChooseSaveMessage::ChooseDirCompleted(choose_dir_res) => {
                    self.choose_save_directory_state.choose_dir_window_open = false;

                    match choose_dir_res {
                        MessageResult::Success(dir) => {
                            self.choose_save_directory_state.saves_dir = dir.clone();

                            return Command::perform(
                                interaction::choose_save_directory::load_files_in_directory(dir),
                                |r| {
                                    Message::ChooseSave(ChooseSaveMessage::LoadedFiles(
                                        MessageResult::handle_result(r),
                                    ))
                                },
                            );
                        }
                        MessageResult::Error(e) => eprintln!("{}", e),
                    }
                }
                ChooseSaveMessage::LoadedFiles(loaded_files) => match loaded_files {
                    MessageResult::Success(mut files) => {
                        files.sort();
                        self.loaded_files = files;

                        self.loaded_files_selected = Box::new(
                            self.loaded_files
                                .get(0)
                                .expect("loaded_files was empty")
                                .clone(),
                        );

                        state_mappers::map_loaded_file_to_state(self);
                    }
                    MessageResult::Error(e) => eprintln!("{}", e),
                },
            },
            Message::ManageSave(manage_save_msg) => match manage_save_msg {
                ManageSaveMessage::General(general_msg) => match general_msg {
                    GeneralMessage::GenerateRandomGuidCompleted(guid) => {
                        self.manage_save_state.main_state.general_state.guid_input = guid;
                    }
                },
                ManageSaveMessage::Character(character_msg) => match character_msg {
                    CharacterMessage::GearMessage(gear_msg) => match gear_msg {
                        CharacterUnlockGearMessage::Grenade(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_grenade_slot = b;
                        }
                        CharacterUnlockGearMessage::Shield(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_shield_slot = b;
                        }
                        CharacterUnlockGearMessage::Weapon1(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_weapon_1_slot = b;
                        }
                        CharacterUnlockGearMessage::Weapon2(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_weapon_2_slot = b;
                        }
                        CharacterUnlockGearMessage::Weapon3(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_weapon_3_slot = b;
                        }
                        CharacterUnlockGearMessage::Weapon4(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_weapon_4_slot = b;
                        }
                        CharacterUnlockGearMessage::Artifact(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_artifact_slot = b;
                        }
                        CharacterUnlockGearMessage::ClassMod(b) => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .gear_state
                                .unlock_class_mod_slot = b;
                        }
                    },
                },
                ManageSaveMessage::FastTravel(fast_travel_msg) => match fast_travel_msg {
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
                },
            },
            Message::SaveFileCompleted(res) => match res {
                MessageResult::Success(_) => {
                    println!("Successfully saved file");

                    return Command::perform(
                        interaction::choose_save_directory::load_files_in_directory(
                            self.choose_save_directory_state.saves_dir.clone(),
                        ),
                        |r| {
                            Message::ChooseSave(ChooseSaveMessage::LoadedFiles(
                                MessageResult::handle_result(r),
                            ))
                        },
                    );
                }
                MessageResult::Error(e) => eprintln!("{}", e),
            },
        };

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let title = Text::new("Borderlands 3 Save Editor".to_uppercase())
            .font(COMPACTA)
            .size(48)
            .color(Color::from_rgb8(242, 203, 5))
            .width(Length::Fill)
            .horizontal_alignment(HorizontalAlignment::Left);

        let mut change_dir_button = Button::new(
            &mut self.change_dir_button_state,
            Text::new("Change Folder")
                .font(JETBRAINS_MONO_BOLD)
                .size(17),
        )
        .padding(10)
        .style(Bl3UiStyle);

        if !self.choose_save_directory_state.choose_dir_window_open {
            change_dir_button =
                change_dir_button.on_press(InteractionMessage::ChooseSaveInteraction(
                    ChooseSaveInteractionMessage::ChooseDirPressed,
                ));
        }

        let all_saves_picklist = PickList::new(
            &mut self.loaded_files_selector,
            &self.loaded_files,
            Some(*self.loaded_files_selected.clone()),
            |f| InteractionMessage::LoadedFileSelected(Box::new(f)),
        )
        .font(JETBRAINS_MONO)
        .text_size(17)
        .width(Length::Fill)
        .padding(10)
        .style(Bl3UiStyle)
        .into_element();

        let save_button = Button::new(
            &mut self.save_file_button_state,
            Text::new("Save").font(JETBRAINS_MONO_BOLD).size(17),
        )
        .on_press(InteractionMessage::ManageSaveInteraction(
            ManageSaveInteractionMessage::SaveFilePressed,
        ))
        .padding(10)
        .style(Bl3UiStyle)
        .into_element();

        let mut menu_bar_content = Row::new()
            .push(title)
            .spacing(15)
            .align_items(Align::Center);

        // mem::discriminant will match any of the enum's under ViewState::ManageSave
        if mem::discriminant(&self.view_state)
            == mem::discriminant(&ViewState::ManageSave(ManageSaveView::TabBar(
                MainTabBarView::General,
            )))
        {
            menu_bar_content = menu_bar_content.push(change_dir_button.into_element());
            menu_bar_content = menu_bar_content.push(all_saves_picklist);
            menu_bar_content = menu_bar_content.push(save_button);
        }

        let menu_bar = Container::new(menu_bar_content)
            .padding(20)
            .width(Length::Fill)
            .style(Bl3UiMenuBarStyle);

        let content = match &self.view_state {
            ViewState::Initializing => views::initialization::view(),
            ViewState::ChooseSaveDirectory => {
                views::choose_save_directory::view(&mut self.choose_save_directory_state)
            }
            ViewState::ManageSave(manage_save_view) => match manage_save_view {
                ManageSaveView::TabBar(main_tab_bar_view) => {
                    views::manage_save::main::view(&mut self.manage_save_state, main_tab_bar_view)
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
