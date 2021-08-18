use std::mem;

use iced::{
    button, pick_list, Align, Application, Button, Clipboard, Color, Column, Command, Container,
    Element, HorizontalAlignment, Length, PickList, Row, Text,
};

use bl3_save_edit_core::bl3_save::bl3_item::{Bl3Item, MAX_BL3_ITEM_PARTS};
use bl3_save_edit_core::bl3_save::sdu::SaveSduSlot;
use bl3_save_edit_core::bl3_save::util::{experience_to_level, REQUIRED_XP_LIST};
use bl3_save_edit_core::file_helper::Bl3FileType;
use bl3_save_edit_core::resources::INVENTORY_SERIAL_DB;

use crate::bl3_ui_style::{Bl3UiContentStyle, Bl3UiMenuBarStyle, Bl3UiStyle};
use crate::commands::{initialization, interaction};
use crate::resources::fonts::{COMPACTA, JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::choose_save_directory::{
    ChooseSaveDirectoryState, ChooseSaveInteractionMessage, ChooseSaveMessage,
};
use crate::views::initialization::InitializationMessage;
use crate::views::manage_save::character::{
    CharacterGearUnlockedMessage, CharacterInteractionMessage, CharacterSduInputChangedMessage,
    CharacterSkinSelectedMessage,
};
use crate::views::manage_save::currency::CurrencyInteractionMessage;
use crate::views::manage_save::general::{GeneralInteractionMessage, GeneralMessage};
use crate::views::manage_save::inventory::{
    available_parts, InventoryInteractionMessage, InventoryStateExt,
};
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
                                    .sdu_unlocker
                                    .backpack
                                    .input = level;
                            }
                            CharacterSduInputChangedMessage::Sniper(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_unlocker
                                    .sniper
                                    .input = level;
                            }
                            CharacterSduInputChangedMessage::Shotgun(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_unlocker
                                    .shotgun
                                    .input = level;
                            }
                            CharacterSduInputChangedMessage::Pistol(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_unlocker
                                    .pistol
                                    .input = level;
                            }
                            CharacterSduInputChangedMessage::Grenade(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_unlocker
                                    .grenade
                                    .input = level;
                            }
                            CharacterSduInputChangedMessage::Smg(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_unlocker
                                    .smg
                                    .input = level;
                            }
                            CharacterSduInputChangedMessage::AssaultRifle(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_unlocker
                                    .assault_rifle
                                    .input = level;
                            }
                            CharacterSduInputChangedMessage::Heavy(level) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .sdu_unlocker
                                    .heavy
                                    .input = level;
                            }
                        },
                        CharacterInteractionMessage::MaxSduSlotsPressed => {
                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_unlocker
                                .backpack
                                .input = SaveSduSlot::Backpack.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_unlocker
                                .sniper
                                .input = SaveSduSlot::Sniper.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_unlocker
                                .shotgun
                                .input = SaveSduSlot::Shotgun.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_unlocker
                                .pistol
                                .input = SaveSduSlot::Pistol.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_unlocker
                                .grenade
                                .input = SaveSduSlot::Grenade.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_unlocker
                                .smg
                                .input = SaveSduSlot::Smg.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_unlocker
                                .assault_rifle
                                .input = SaveSduSlot::Ar.maximum();

                            self.manage_save_state
                                .main_state
                                .character_state
                                .sdu_unlocker
                                .heavy
                                .input = SaveSduSlot::Heavy.maximum();
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
                                        .skin_selectors
                                        .head_skin
                                        .selected = selected;
                                }
                                CharacterSkinSelectedMessage::CharacterSkin(selected) => {
                                    self.manage_save_state
                                        .main_state
                                        .character_state
                                        .skin_selectors
                                        .character_skin
                                        .selected = selected;
                                }
                                CharacterSkinSelectedMessage::EchoTheme(selected) => {
                                    self.manage_save_state
                                        .main_state
                                        .character_state
                                        .skin_selectors
                                        .echo_theme
                                        .selected = selected;
                                }
                            }
                        }
                        CharacterInteractionMessage::GearMessage(gear_msg) => match gear_msg {
                            CharacterGearUnlockedMessage::Grenade(b) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .gear_unlocker
                                    .grenade
                                    .is_unlocked = b;
                            }
                            CharacterGearUnlockedMessage::Shield(b) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .gear_unlocker
                                    .shield
                                    .is_unlocked = b;
                            }
                            CharacterGearUnlockedMessage::Weapon1(b) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .gear_unlocker
                                    .weapon_1
                                    .is_unlocked = b;
                            }
                            CharacterGearUnlockedMessage::Weapon2(b) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .gear_unlocker
                                    .weapon_2
                                    .is_unlocked = b;
                            }
                            CharacterGearUnlockedMessage::Weapon3(b) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .gear_unlocker
                                    .weapon_3
                                    .is_unlocked = b;
                            }
                            CharacterGearUnlockedMessage::Weapon4(b) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .gear_unlocker
                                    .weapon_4
                                    .is_unlocked = b;
                            }
                            CharacterGearUnlockedMessage::Artifact(b) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .gear_unlocker
                                    .artifact
                                    .is_unlocked = b;
                            }
                            CharacterGearUnlockedMessage::ClassMod(b) => {
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .gear_unlocker
                                    .class_mod
                                    .is_unlocked = b;
                            }
                        },
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
                    ManageSaveInteractionMessage::SaveFilePressed => {
                        let current_file = &mut self.manage_save_state.current_file;

                        // current_file.character_data.set_head_skin_selected(
                        //     &self
                        //         .manage_save_state
                        //         .main_state
                        //         .character_state
                        //         .skin_state
                        //         .head_skin_selected,
                        // );

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
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .selected_item_index = item_index;

                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .map_current_item_if_exists(|i| {
                                    i.editor.available_parts.parts_index =
                                        available_parts::AvailablePartsIndex {
                                            category_index: 0,
                                            part_index: 0,
                                        }
                                });
                        }
                        InventoryInteractionMessage::AvailablePartPressed(
                            available_parts_index,
                        ) => {
                            let selected_item_index = self
                                .manage_save_state
                                .main_state
                                .inventory_state
                                .selected_item_index;

                            if let Some(current_item) = self
                                .manage_save_state
                                .main_state
                                .inventory_state
                                .items_mut()
                                .get_mut(selected_item_index)
                            {
                                if let Some(item_parts) = &mut current_item.item.item_parts {
                                    if item_parts.parts().len() < MAX_BL3_ITEM_PARTS {
                                        let part_selected = current_item
                                            .editor
                                            .available_parts
                                            .parts
                                            .get(available_parts_index.category_index)
                                            .and_then(|p| {
                                                p.parts.get(available_parts_index.part_index)
                                            });

                                        if let Some(part_selected) = part_selected {
                                            let part_inv_key = &item_parts.part_inv_key;

                                            if let Ok(bl3_part) = INVENTORY_SERIAL_DB
                                                .get_part_by_short_name(
                                                    part_inv_key,
                                                    &part_selected.part.name,
                                                )
                                            {
                                                if let Err(e) = current_item.item.add_part(bl3_part)
                                                {
                                                    eprintln!("{}", e);
                                                }

                                                self.manage_save_state
                                                    .main_state
                                                    .inventory_state
                                                    .map_current_item_if_exists(|i| {
                                                        i.editor.available_parts.parts_index =
                                                            available_parts_index
                                                    });
                                            }
                                        }
                                    }
                                }
                            } else {
                            }
                        }
                        InventoryInteractionMessage::CurrentPartPressed(current_parts_index) => {
                            let selected_item_index = self
                                .manage_save_state
                                .main_state
                                .inventory_state
                                .selected_item_index;

                            if let Some(current_item) = self
                                .manage_save_state
                                .main_state
                                .inventory_state
                                .items_mut()
                                .get_mut(selected_item_index)
                            {
                                let part_selected = current_item
                                    .editor
                                    .current_parts
                                    .parts
                                    .get(current_parts_index.category_index)
                                    .and_then(|p| p.parts.get(current_parts_index.part_index));

                                if let Some(part_selected) = part_selected {
                                    if let Err(e) =
                                        current_item.item.remove_part(&part_selected.part)
                                    {
                                        eprintln!("{}", e);
                                    }

                                    self.manage_save_state
                                        .main_state
                                        .inventory_state
                                        .map_current_item_if_exists_to_editor_state();
                                }
                            }
                        }
                        InventoryInteractionMessage::ImportItemInputChanged(s) => {
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .import_serial_input = s;
                        }
                        InventoryInteractionMessage::ImportItemFromSerial => {
                            let item_serial = self
                                .manage_save_state
                                .main_state
                                .inventory_state
                                .import_serial_input
                                .trim();

                            match Bl3Item::from_serial_base64(item_serial) {
                                Ok(bl3_item) => {
                                    self.manage_save_state
                                        .main_state
                                        .inventory_state
                                        .add_item(bl3_item);

                                    self.manage_save_state
                                        .main_state
                                        .inventory_state
                                        .selected_item_index = self
                                        .manage_save_state
                                        .main_state
                                        .inventory_state
                                        .items()
                                        .len()
                                        - 1;

                                    self.manage_save_state
                                        .main_state
                                        .inventory_state
                                        .map_current_item_if_exists_to_editor_state();

                                    self.manage_save_state
                                        .main_state
                                        .inventory_state
                                        .item_list_scrollable_state
                                        .snap_to(1.0)
                                }
                                Err(e) => eprintln!("{}", e),
                            }
                        }
                        InventoryInteractionMessage::SyncItemLevelWithCharacterLevel => {
                            let character_level =
                                self.manage_save_state
                                    .main_state
                                    .character_state
                                    .xp_level_input as usize;

                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .map_current_item_if_exists(|i| {
                                    if let Err(e) = i.item.set_level(character_level) {
                                        eprintln!("{}", e);
                                    }
                                });
                        }
                        InventoryInteractionMessage::ItemLevelInputChanged(item_level_input) => {
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .map_current_item_if_exists(|i| {
                                    if let Err(e) = i.item.set_level(item_level_input as usize) {
                                        eprintln!("{}", e);
                                    }
                                });
                        }
                        InventoryInteractionMessage::DeleteItem(id) => {
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .remove_item(id);

                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .selected_item_index = 0;

                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .map_current_item_if_exists_to_editor_state();

                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .item_list_scrollable_state
                                .snap_to(0.0)
                        }
                        InventoryInteractionMessage::BalanceInputSelected(balance_selected) => {
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .map_current_item_if_exists(|i| {
                                    if let Err(e) = i.item.set_balance(balance_selected) {
                                        eprintln!("{}", e);
                                    };
                                });
                        }
                        InventoryInteractionMessage::InvDataInputSelected(inv_data_selected) => {
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .map_current_item_if_exists(|i| {
                                    if let Err(e) = i.item.set_inv_data(inv_data_selected) {
                                        eprintln!("{}", e);
                                    }
                                });
                        }
                        InventoryInteractionMessage::ManufacturerInputSelected(
                            manufacturer_selected,
                        ) => {
                            self.manage_save_state
                                .main_state
                                .inventory_state
                                .map_current_item_if_exists(|i| {
                                    if let Err(e) = i.item.set_manufacturer(manufacturer_selected) {
                                        eprintln!("{}", e);
                                    }
                                });
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
