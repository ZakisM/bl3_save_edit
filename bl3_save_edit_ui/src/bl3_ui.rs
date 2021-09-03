use std::mem;
use std::path::PathBuf;

use iced::{
    button, pick_list, Align, Application, Button, Clipboard, Color, Column, Command, Container,
    Element, HorizontalAlignment, Length, PickList, Row, Text,
};

use bl3_save_edit_core::bl3_profile::sdu::ProfileSduSlot;
use bl3_save_edit_core::bl3_profile::Bl3Profile;
use bl3_save_edit_core::bl3_save::ammo::AmmoPool;
use bl3_save_edit_core::bl3_save::sdu::SaveSduSlot;
use bl3_save_edit_core::bl3_save::util::{experience_to_level, REQUIRED_XP_LIST};
use bl3_save_edit_core::bl3_save::Bl3Save;
use bl3_save_edit_core::file_helper::Bl3FileType;
use bl3_save_edit_core::parser::HeaderType;

use crate::bl3_ui_style::{Bl3UiContentStyle, Bl3UiMenuBarStyle, Bl3UiStyle};
use crate::commands::{initialization, interaction};
use crate::config::{Config, ConfigMessage};
use crate::resources::fonts::{COMPACTA, JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::state_mappers::{manage_profile, manage_save};
use crate::views::choose_save_directory::{
    ChooseSaveDirectoryState, ChooseSaveInteractionMessage, ChooseSaveMessage,
};
use crate::views::initialization::InitializationMessage;
use crate::views::item_editor::ItemEditorFileType;
use crate::views::manage_profile::bank::ProfileBankInteractionMessage;
use crate::views::manage_profile::general::ProfileGeneralInteractionMessage;
use crate::views::manage_profile::keys::ProfileKeysInteractionMessage;
use crate::views::manage_profile::main::{ProfileTabBarInteractionMessage, ProfileTabBarView};
use crate::views::manage_profile::profile::{
    ProfileProfileInteractionMessage, ProfileSduMessage, ProfileSkinUnlockedMessage,
};
use crate::views::manage_profile::{
    ManageProfileInteractionMessage, ManageProfileState, ManageProfileView,
};
use crate::views::manage_save::character::{
    CharacterAmmoMessage, CharacterGearUnlockedMessage, CharacterSduMessage,
    CharacterSkinSelectedMessage, SaveCharacterInteractionMessage,
};
use crate::views::manage_save::currency::SaveCurrencyInteractionMessage;
use crate::views::manage_save::general::{GeneralMessage, SaveGeneralInteractionMessage};
use crate::views::manage_save::inventory::SaveInventoryInteractionMessage;
use crate::views::manage_save::main::{SaveTabBarInteractionMessage, SaveTabBarView};
use crate::views::manage_save::{
    ManageSaveInteractionMessage, ManageSaveMessage, ManageSaveState, ManageSaveView,
};
use crate::views::InteractionExt;
use crate::widgets::notification::{Notification, NotificationSentiment};
use crate::{state_mappers, views, VERSION};

#[derive(Debug, Default)]
pub struct Bl3Ui {
    pub config: Config,
    pub view_state: ViewState,
    choose_save_directory_state: ChooseSaveDirectoryState,
    pub manage_save_state: ManageSaveState,
    pub manage_profile_state: ManageProfileState,
    loaded_files_selector: pick_list::State<Bl3FileType>,
    pub loaded_files_selected: Box<Bl3FileType>,
    loaded_files: Vec<Bl3FileType>,
    change_dir_button_state: button::State,
    save_file_button_state: button::State,
    notification: Option<Notification>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Initialization(InitializationMessage),
    Config(ConfigMessage),
    Interaction(InteractionMessage),
    ChooseSave(ChooseSaveMessage),
    ManageSave(ManageSaveMessage),
    SaveFileCompleted(MessageResult<Bl3Save>),
    SaveProfileCompleted(MessageResult<Bl3Profile>),
    ClearNotification,
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
    ManageProfileInteraction(ManageProfileInteractionMessage),
    LoadedFileSelected(Box<Bl3FileType>),
    Ignore,
}

#[derive(Debug, PartialEq)]
pub enum ViewState {
    Initializing,
    Loading,
    ChooseSaveDirectory,
    ManageSave(ManageSaveView),
    ManageProfile(ManageProfileView),
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
                view_state: ViewState::Initializing,
                ..Bl3Ui::default()
            },
            Command::perform(initialization::load_lazy_data(), |_| {
                Message::Initialization(InitializationMessage::LazyData)
            }),
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
                InitializationMessage::LazyData => {
                    //Load config once initialized
                    return Command::perform(initialization::load_config(), |r| {
                        Message::Initialization(InitializationMessage::Config(r))
                    });
                }
                InitializationMessage::Config(config) => {
                    self.config = config;

                    if self.config.saves_dir().exists() {
                        return Command::perform(
                            interaction::choose_save_directory::load_files_in_directory(
                                self.config.saves_dir().to_path_buf(),
                            ),
                            |r| {
                                Message::ChooseSave(ChooseSaveMessage::FilesLoaded(
                                    MessageResult::handle_result(r),
                                ))
                            },
                        );
                    } else if *self.config.saves_dir() != PathBuf::default() {
                        self.notification = Some(Notification::new("Failed to load your previously selected Save/Profile directory. Please select another directory.", NotificationSentiment::Negative));
                    }

                    self.view_state = ViewState::ChooseSaveDirectory;
                }
            },
            Message::Config(config_msg) => match config_msg {
                ConfigMessage::SaveCompleted(res) => match res {
                    MessageResult::Success(_) => println!("Successfully saved config."),
                    MessageResult::Error(e) => eprintln!("Failed to save config: {}", e),
                },
            },
            Message::Interaction(interaction_msg) => {
                self.notification = None;

                match interaction_msg {
                    InteractionMessage::ChooseSaveInteraction(choose_save_msg) => {
                        return match choose_save_msg {
                            ChooseSaveInteractionMessage::ChooseDirPressed => {
                                self.choose_save_directory_state.choose_dir_window_open = true;

                                Command::perform(
                                    interaction::choose_save_directory::choose(
                                        self.config.saves_dir().to_path_buf(),
                                    ),
                                    |r| {
                                        Message::ChooseSave(ChooseSaveMessage::ChooseDirCompleted(
                                            MessageResult::handle_result(r),
                                        ))
                                    },
                                )
                            }
                        };
                    }
                    InteractionMessage::ManageSaveInteraction(manage_save_msg) => {
                        match manage_save_msg {
                            ManageSaveInteractionMessage::TabBar(tab_bar_msg) => {
                                match tab_bar_msg {
                                    SaveTabBarInteractionMessage::General => {
                                        self.view_state = ViewState::ManageSave(
                                            ManageSaveView::TabBar(SaveTabBarView::General),
                                        )
                                    }
                                    SaveTabBarInteractionMessage::Character => {
                                        self.view_state = ViewState::ManageSave(
                                            ManageSaveView::TabBar(SaveTabBarView::Character),
                                        )
                                    }
                                    SaveTabBarInteractionMessage::Inventory => {
                                        self.view_state = ViewState::ManageSave(
                                            ManageSaveView::TabBar(SaveTabBarView::Inventory),
                                        )
                                    }
                                    SaveTabBarInteractionMessage::Currency => {
                                        self.view_state = ViewState::ManageSave(
                                            ManageSaveView::TabBar(SaveTabBarView::Currency),
                                        )
                                    }
                                }
                            }
                            ManageSaveInteractionMessage::General(general_msg) => match general_msg
                            {
                                SaveGeneralInteractionMessage::Guid(guid) => {
                                    self.manage_save_state
                                        .save_view_state
                                        .general_state
                                        .guid_input = guid;
                                }
                                SaveGeneralInteractionMessage::Slot(slot) => {
                                    let filename = format!("{}.sav", slot);

                                    self.manage_save_state
                                        .save_view_state
                                        .general_state
                                        .slot_input = slot;
                                    self.manage_save_state
                                        .save_view_state
                                        .general_state
                                        .filename_input = filename.clone();
                                    self.manage_save_state.current_file.file_name = filename;
                                }
                                SaveGeneralInteractionMessage::GenerateGuidPressed => {
                                    return Command::perform(
                                        interaction::manage_save::general::generate_random_guid(),
                                        |r| {
                                            Message::ManageSave(ManageSaveMessage::General(
                                                GeneralMessage::GenerateRandomGuidCompleted(r),
                                            ))
                                        },
                                    );
                                }
                                SaveGeneralInteractionMessage::SaveTypeSelected(save_type) => {
                                    self.manage_save_state
                                        .save_view_state
                                        .general_state
                                        .save_type_selected = save_type;
                                }
                            },
                            ManageSaveInteractionMessage::Character(character_msg) => {
                                match character_msg {
                                    SaveCharacterInteractionMessage::Name(name_input) => {
                                        self.manage_save_state
                                            .save_view_state
                                            .character_state
                                            .name_input = name_input;
                                    }
                                    SaveCharacterInteractionMessage::XpLevel(level) => {
                                        let xp_points = if level > 0 {
                                            REQUIRED_XP_LIST[level as usize - 1][0]
                                        } else {
                                            0
                                        };

                                        let character_state = &mut self
                                            .manage_save_state
                                            .save_view_state
                                            .character_state;

                                        character_state.level_input = level;

                                        character_state.experience_points_input = xp_points;
                                    }
                                    SaveCharacterInteractionMessage::XpPoints(xp) => {
                                        let level = experience_to_level(xp as i32).unwrap_or(1);

                                        let character_state = &mut self
                                            .manage_save_state
                                            .save_view_state
                                            .character_state;

                                        character_state.experience_points_input = xp;

                                        character_state.level_input = level;
                                    }
                                    SaveCharacterInteractionMessage::SduMessage(sdu_message) => {
                                        let sdu_unlocker = &mut self
                                            .manage_save_state
                                            .save_view_state
                                            .character_state
                                            .sdu_unlocker;

                                        match sdu_message {
                                            CharacterSduMessage::Backpack(level) => {
                                                sdu_unlocker.backpack.input = level;
                                            }
                                            CharacterSduMessage::Sniper(level) => {
                                                sdu_unlocker.sniper.input = level;
                                            }
                                            CharacterSduMessage::Shotgun(level) => {
                                                sdu_unlocker.shotgun.input = level;
                                            }
                                            CharacterSduMessage::Pistol(level) => {
                                                sdu_unlocker.pistol.input = level;
                                            }
                                            CharacterSduMessage::Grenade(level) => {
                                                sdu_unlocker.grenade.input = level;
                                            }
                                            CharacterSduMessage::Smg(level) => {
                                                sdu_unlocker.smg.input = level;
                                            }
                                            CharacterSduMessage::AssaultRifle(level) => {
                                                sdu_unlocker.assault_rifle.input = level;
                                            }
                                            CharacterSduMessage::Heavy(level) => {
                                                sdu_unlocker.heavy.input = level;
                                            }
                                        }
                                    }
                                    SaveCharacterInteractionMessage::MaxSduSlotsPressed => {
                                        let sdu_unlocker = &mut self
                                            .manage_save_state
                                            .save_view_state
                                            .character_state
                                            .sdu_unlocker;

                                        sdu_unlocker.backpack.input =
                                            SaveSduSlot::Backpack.maximum();

                                        sdu_unlocker.sniper.input = SaveSduSlot::Sniper.maximum();

                                        sdu_unlocker.shotgun.input = SaveSduSlot::Shotgun.maximum();

                                        sdu_unlocker.pistol.input = SaveSduSlot::Pistol.maximum();

                                        sdu_unlocker.grenade.input = SaveSduSlot::Grenade.maximum();

                                        sdu_unlocker.smg.input = SaveSduSlot::Smg.maximum();

                                        sdu_unlocker.assault_rifle.input =
                                            SaveSduSlot::Ar.maximum();

                                        sdu_unlocker.heavy.input = SaveSduSlot::Heavy.maximum();
                                    }
                                    SaveCharacterInteractionMessage::AmmoMessage(ammo_message) => {
                                        let ammo_setter = &mut self
                                            .manage_save_state
                                            .save_view_state
                                            .character_state
                                            .ammo_setter;

                                        match ammo_message {
                                            CharacterAmmoMessage::Sniper(amount) => {
                                                ammo_setter.sniper.input = amount;
                                            }
                                            CharacterAmmoMessage::Shotgun(amount) => {
                                                ammo_setter.shotgun.input = amount;
                                            }
                                            CharacterAmmoMessage::Pistol(amount) => {
                                                ammo_setter.pistol.input = amount;
                                            }
                                            CharacterAmmoMessage::Grenade(amount) => {
                                                ammo_setter.grenade.input = amount;
                                            }
                                            CharacterAmmoMessage::Smg(amount) => {
                                                ammo_setter.smg.input = amount;
                                            }
                                            CharacterAmmoMessage::AssaultRifle(amount) => {
                                                ammo_setter.assault_rifle.input = amount;
                                            }
                                            CharacterAmmoMessage::Heavy(amount) => {
                                                ammo_setter.heavy.input = amount;
                                            }
                                        }
                                    }
                                    SaveCharacterInteractionMessage::MaxAmmoAmountsPressed => {
                                        let ammo_setter = &mut self
                                            .manage_save_state
                                            .save_view_state
                                            .character_state
                                            .ammo_setter;

                                        ammo_setter.sniper.input = AmmoPool::Sniper.maximum();

                                        ammo_setter.shotgun.input = AmmoPool::Shotgun.maximum();

                                        ammo_setter.pistol.input = AmmoPool::Pistol.maximum();

                                        ammo_setter.grenade.input = AmmoPool::Grenade.maximum();

                                        ammo_setter.smg.input = AmmoPool::Smg.maximum();

                                        ammo_setter.assault_rifle.input = AmmoPool::Ar.maximum();

                                        ammo_setter.heavy.input = AmmoPool::Heavy.maximum();
                                    }
                                    SaveCharacterInteractionMessage::PlayerClassSelected(
                                        player_class,
                                    ) => {
                                        self.manage_save_state
                                            .save_view_state
                                            .character_state
                                            .player_class_selected_class = player_class;
                                    }
                                    SaveCharacterInteractionMessage::SkinMessage(skin_message) => {
                                        let skin_selectors = &mut self
                                            .manage_save_state
                                            .save_view_state
                                            .character_state
                                            .skin_selectors;

                                        match skin_message {
                                            CharacterSkinSelectedMessage::HeadSkin(selected) => {
                                                skin_selectors.head_skin.selected = selected;
                                            }
                                            CharacterSkinSelectedMessage::CharacterSkin(
                                                selected,
                                            ) => {
                                                skin_selectors.character_skin.selected = selected;
                                            }
                                            CharacterSkinSelectedMessage::EchoTheme(selected) => {
                                                skin_selectors.echo_theme.selected = selected;
                                            }
                                        }
                                    }
                                    SaveCharacterInteractionMessage::GearMessage(gear_msg) => {
                                        let gear_unlocker = &mut self
                                            .manage_save_state
                                            .save_view_state
                                            .character_state
                                            .gear_unlocker;

                                        match gear_msg {
                                            CharacterGearUnlockedMessage::Grenade(b) => {
                                                gear_unlocker.grenade.is_unlocked = b;
                                            }
                                            CharacterGearUnlockedMessage::Shield(b) => {
                                                gear_unlocker.shield.is_unlocked = b;
                                            }
                                            CharacterGearUnlockedMessage::Weapon1(b) => {
                                                gear_unlocker.weapon_1.is_unlocked = b;
                                            }
                                            CharacterGearUnlockedMessage::Weapon2(b) => {
                                                gear_unlocker.weapon_2.is_unlocked = b;
                                            }
                                            CharacterGearUnlockedMessage::Weapon3(b) => {
                                                gear_unlocker.weapon_3.is_unlocked = b;
                                            }
                                            CharacterGearUnlockedMessage::Weapon4(b) => {
                                                gear_unlocker.weapon_4.is_unlocked = b;
                                            }
                                            CharacterGearUnlockedMessage::Artifact(b) => {
                                                gear_unlocker.artifact.is_unlocked = b;
                                            }
                                            CharacterGearUnlockedMessage::ClassMod(b) => {
                                                gear_unlocker.class_mod.is_unlocked = b;
                                            }
                                        }
                                    }
                                }
                            }
                            ManageSaveInteractionMessage::Currency(currency_msg) => {
                                match currency_msg {
                                    SaveCurrencyInteractionMessage::Money(money) => {
                                        self.manage_save_state
                                            .save_view_state
                                            .currency_state
                                            .money_input = money;
                                    }
                                    SaveCurrencyInteractionMessage::Eridium(eridium) => {
                                        self.manage_save_state
                                            .save_view_state
                                            .currency_state
                                            .eridium_input = eridium;
                                    }
                                    SaveCurrencyInteractionMessage::MaxMoneyPressed => {
                                        self.manage_save_state
                                            .save_view_state
                                            .currency_state
                                            .money_input = i32::MAX;
                                    }
                                    SaveCurrencyInteractionMessage::MaxEridiumPressed => {
                                        self.manage_save_state
                                            .save_view_state
                                            .currency_state
                                            .eridium_input = i32::MAX;
                                    }
                                }
                            }
                            ManageSaveInteractionMessage::Inventory(inventory_msg) => {
                                match inventory_msg {
                                    SaveInventoryInteractionMessage::Editor(
                                        item_editor_message,
                                    ) => {
                                        let notification = item_editor_message.update_state(
                                            &mut self
                                                .manage_save_state
                                                .save_view_state
                                                .inventory_state
                                                .item_editor_state,
                                            ItemEditorFileType::Save(
                                                &mut self.manage_save_state.current_file,
                                            ),
                                        );

                                        self.notification = notification;
                                    }
                                }
                            }
                            ManageSaveInteractionMessage::SaveFilePressed => {
                                //Lets not make any modifications to the current file just in case we have any errors
                                let mut current_file = self.manage_save_state.current_file.clone();

                                if let Err(e) = manage_save::map_all_states_to_save(
                                    &mut self.manage_save_state,
                                    &mut current_file,
                                ) {
                                    let msg = format!("Failed to save file: {}", e);

                                    self.notification = Some(Notification::new(
                                        msg,
                                        NotificationSentiment::Negative,
                                    ));

                                    return Command::none();
                                }

                                let output_file = self
                                    .choose_save_directory_state
                                    .saves_dir
                                    .join(&self.manage_save_state.current_file.file_name);

                                match current_file.to_bytes() {
                                    Ok((output, save_file)) => {
                                        return Command::perform(
                                            interaction::save::save_file(
                                                output_file,
                                                output,
                                                save_file,
                                            ),
                                            |r| {
                                                Message::SaveFileCompleted(
                                                    MessageResult::handle_result(r),
                                                )
                                            },
                                        );
                                    }
                                    Err(e) => {
                                        let msg = format!("Failed to save file: {}", e);

                                        self.notification = Some(Notification::new(
                                            msg,
                                            NotificationSentiment::Negative,
                                        ));
                                    }
                                };
                            }
                        }
                    }
                    InteractionMessage::ManageProfileInteraction(manage_profile_msg) => {
                        match manage_profile_msg {
                            ManageProfileInteractionMessage::TabBar(tab_bar_msg) => {
                                match tab_bar_msg {
                                    ProfileTabBarInteractionMessage::General => {
                                        self.view_state = ViewState::ManageProfile(
                                            ManageProfileView::TabBar(ProfileTabBarView::General),
                                        )
                                    }
                                    ProfileTabBarInteractionMessage::Profile => {
                                        self.view_state = ViewState::ManageProfile(
                                            ManageProfileView::TabBar(ProfileTabBarView::Profile),
                                        )
                                    }
                                    ProfileTabBarInteractionMessage::Keys => {
                                        self.view_state = ViewState::ManageProfile(
                                            ManageProfileView::TabBar(ProfileTabBarView::Keys),
                                        )
                                    }
                                    ProfileTabBarInteractionMessage::Bank => {
                                        self.view_state = ViewState::ManageProfile(
                                            ManageProfileView::TabBar(ProfileTabBarView::Bank),
                                        )
                                    }
                                }
                            }
                            ManageProfileInteractionMessage::General(general_msg) => {
                                match general_msg {
                                    ProfileGeneralInteractionMessage::ProfileTypeSelected(
                                        profile_type,
                                    ) => {
                                        self.manage_profile_state
                                            .profile_view_state
                                            .general_state
                                            .profile_type_selected = profile_type;
                                    }
                                }
                            }
                            ManageProfileInteractionMessage::Profile(profile_msg) => {
                                match profile_msg {
                                    ProfileProfileInteractionMessage::GuardianRank(
                                        guardian_rank,
                                    ) => {
                                        self.manage_profile_state
                                            .profile_view_state
                                            .profile_state
                                            .guardian_rank_input = guardian_rank;
                                    }
                                    ProfileProfileInteractionMessage::GuardianRankTokens(
                                        guardian_rank_tokens,
                                    ) => {
                                        self.manage_profile_state
                                            .profile_view_state
                                            .profile_state
                                            .guardian_rank_tokens_input = guardian_rank_tokens;
                                    }
                                    ProfileProfileInteractionMessage::ScienceLevelSelected(
                                        science_level,
                                    ) => {
                                        self.manage_profile_state
                                            .profile_view_state
                                            .profile_state
                                            .science_level_selected = science_level;
                                    }
                                    ProfileProfileInteractionMessage::ScienceTokens(
                                        science_level_tokens,
                                    ) => {
                                        self.manage_profile_state
                                            .profile_view_state
                                            .profile_state
                                            .science_tokens_input = science_level_tokens;
                                    }
                                    ProfileProfileInteractionMessage::SkinMessage(skin_message) => {
                                        let skin_unlocker = &mut self
                                            .manage_profile_state
                                            .profile_view_state
                                            .profile_state
                                            .skin_unlocker;

                                        match skin_message {
                                            ProfileSkinUnlockedMessage::CharacterSkins(
                                                selected,
                                            ) => {
                                                skin_unlocker.character_skins.is_unlocked =
                                                    selected;
                                            }
                                            ProfileSkinUnlockedMessage::CharacterHeads(
                                                selected,
                                            ) => {
                                                skin_unlocker.character_heads.is_unlocked =
                                                    selected;
                                            }
                                            ProfileSkinUnlockedMessage::EchoThemes(selected) => {
                                                skin_unlocker.echo_themes.is_unlocked = selected;
                                            }
                                            ProfileSkinUnlockedMessage::Emotes(selected) => {
                                                skin_unlocker.emotes.is_unlocked = selected;
                                            }
                                            ProfileSkinUnlockedMessage::RoomDecorations(
                                                selected,
                                            ) => {
                                                skin_unlocker.room_decorations.is_unlocked =
                                                    selected;
                                            }
                                            ProfileSkinUnlockedMessage::WeaponSkins(selected) => {
                                                skin_unlocker.weapon_skins.is_unlocked = selected;
                                            }
                                            ProfileSkinUnlockedMessage::WeaponTrinkets(
                                                selected,
                                            ) => {
                                                skin_unlocker.weapon_trinkets.is_unlocked =
                                                    selected;
                                            }
                                        }
                                    }
                                    ProfileProfileInteractionMessage::SduMessage(sdu_message) => {
                                        let sdu_unlocker = &mut self
                                            .manage_profile_state
                                            .profile_view_state
                                            .profile_state
                                            .sdu_unlocker;

                                        match sdu_message {
                                            ProfileSduMessage::Bank(level) => {
                                                sdu_unlocker.bank.input = level;
                                            }
                                            ProfileSduMessage::LostLoot(level) => {
                                                sdu_unlocker.lost_loot.input = level;
                                            }
                                        }
                                    }
                                    ProfileProfileInteractionMessage::MaxSduSlotsPressed => {
                                        let sdu_unlocker = &mut self
                                            .manage_profile_state
                                            .profile_view_state
                                            .profile_state
                                            .sdu_unlocker;

                                        sdu_unlocker.bank.input = ProfileSduSlot::Bank.maximum();

                                        sdu_unlocker.lost_loot.input =
                                            ProfileSduSlot::LostLoot.maximum();
                                    }
                                }
                            }
                            ManageProfileInteractionMessage::Keys(keys_message) => {
                                let keys_state =
                                    &mut self.manage_profile_state.profile_view_state.keys_state;

                                match keys_message {
                                    ProfileKeysInteractionMessage::GoldenKeys(golden_keys) => {
                                        keys_state.golden_keys_input = golden_keys;
                                    }
                                    ProfileKeysInteractionMessage::DiamondKeys(diamond_keys) => {
                                        keys_state.diamond_keys_input = diamond_keys;
                                    }
                                    ProfileKeysInteractionMessage::VaultCard1Keys(
                                        vault_card_1_keys,
                                    ) => {
                                        keys_state.vault_card_1_keys_input = vault_card_1_keys;
                                    }
                                    ProfileKeysInteractionMessage::VaultCard1Chests(
                                        vault_card_1_chests,
                                    ) => {
                                        keys_state.vault_card_1_chests_input = vault_card_1_chests;
                                    }
                                    ProfileKeysInteractionMessage::MaxGoldenKeysPressed => {
                                        keys_state.golden_keys_input = i32::MAX;
                                    }
                                    ProfileKeysInteractionMessage::MaxDiamondKeysPressed => {
                                        keys_state.diamond_keys_input = i32::MAX;
                                    }
                                    ProfileKeysInteractionMessage::MaxVaultCard1KeysPressed => {
                                        keys_state.vault_card_1_keys_input = i32::MAX;
                                    }
                                    ProfileKeysInteractionMessage::MaxVaultCard1ChestsPressed => {
                                        keys_state.vault_card_1_chests_input = i32::MAX;
                                    }
                                }
                            }
                            ManageProfileInteractionMessage::Bank(bank_message) => {
                                match bank_message {
                                    ProfileBankInteractionMessage::Editor(item_editor_message) => {
                                        let notification = item_editor_message.update_state(
                                            &mut self
                                                .manage_profile_state
                                                .profile_view_state
                                                .bank_state
                                                .item_editor_state,
                                            ItemEditorFileType::ProfileBank(
                                                &mut self.manage_profile_state.current_file,
                                            ),
                                        );

                                        self.notification = notification;
                                    }
                                }
                            }
                            ManageProfileInteractionMessage::SaveProfilePressed => {
                                //Lets not make any modifications to the current file just in case we have any errors
                                let mut current_file =
                                    self.manage_profile_state.current_file.clone();

                                if let Err(e) = manage_profile::map_all_states_to_profile(
                                    &mut self.manage_profile_state,
                                    &mut current_file,
                                ) {
                                    let msg = format!("Failed to save profile: {}", e);

                                    self.notification = Some(Notification::new(
                                        msg,
                                        NotificationSentiment::Negative,
                                    ));

                                    return Command::none();
                                }

                                let output_file = self
                                    .choose_save_directory_state
                                    .saves_dir
                                    .join(&self.manage_profile_state.current_file.file_name);

                                match current_file.to_bytes() {
                                    Ok((output, profile)) => {
                                        return Command::perform(
                                            interaction::save::save_profile(
                                                output_file,
                                                output,
                                                profile,
                                            ),
                                            |r| {
                                                Message::SaveProfileCompleted(
                                                    MessageResult::handle_result(r),
                                                )
                                            },
                                        );
                                    }
                                    Err(e) => {
                                        let msg = format!("Failed to save file: {}", e);

                                        self.notification = Some(Notification::new(
                                            msg,
                                            NotificationSentiment::Negative,
                                        ));
                                    }
                                };
                            }
                        }
                    }
                    InteractionMessage::LoadedFileSelected(loaded_file) => {
                        self.loaded_files_selected = loaded_file;

                        state_mappers::map_loaded_file_to_state(self);
                    }
                    InteractionMessage::Ignore => {}
                }
            }
            Message::ChooseSave(choose_save_msg) => match choose_save_msg {
                ChooseSaveMessage::ChooseDirCompleted(choose_dir_res) => {
                    self.choose_save_directory_state.choose_dir_window_open = false;

                    match choose_dir_res {
                        MessageResult::Success(dir) => {
                            self.choose_save_directory_state.saves_dir = dir.clone();

                            self.view_state = ViewState::Loading;

                            return Command::perform(
                                interaction::choose_save_directory::load_files_in_directory(dir),
                                |r| {
                                    Message::ChooseSave(ChooseSaveMessage::FilesLoaded(
                                        MessageResult::handle_result(r),
                                    ))
                                },
                            );
                        }
                        MessageResult::Error(e) => {
                            let msg = format!("Failed to choose save directory: {}", e);

                            self.notification =
                                Some(Notification::new(msg, NotificationSentiment::Negative));
                        }
                    }
                }
                ChooseSaveMessage::FilesLoaded(res) => match res {
                    MessageResult::Success((dir, mut files)) => {
                        files.sort();
                        self.loaded_files = files;

                        self.loaded_files_selected = Box::new(
                            self.loaded_files
                                .get(0)
                                .expect("loaded_files was empty")
                                .clone(),
                        );

                        state_mappers::map_loaded_file_to_state(self);

                        self.config.set_saves_dir(dir);

                        return Command::perform(self.config.clone().save(), |r| {
                            Message::Config(ConfigMessage::SaveCompleted(
                                MessageResult::handle_result(r),
                            ))
                        });
                    }
                    MessageResult::Error(e) => {
                        let msg = format!("Failed to load save directory: {}", e);

                        self.view_state = ViewState::ChooseSaveDirectory;

                        self.notification =
                            Some(Notification::new(msg, NotificationSentiment::Negative));
                    }
                },
            },
            Message::ManageSave(manage_save_msg) => match manage_save_msg {
                ManageSaveMessage::General(general_msg) => match general_msg {
                    GeneralMessage::GenerateRandomGuidCompleted(guid) => {
                        self.manage_save_state
                            .save_view_state
                            .general_state
                            .guid_input = guid;
                    }
                },
            },
            Message::SaveFileCompleted(res) => match res {
                MessageResult::Success(save) => {
                    self.notification = Some(Notification::new(
                        "Successfully saved file!",
                        NotificationSentiment::Positive,
                    ));

                    let loaded_file_selected = &*self.loaded_files_selected;

                    let loaded_file = self
                        .loaded_files
                        .iter_mut()
                        .find(|f| *f == loaded_file_selected)
                        .expect("failed to find loaded file");

                    self.manage_save_state.current_file = save.clone();

                    let save_file_name = save.file_name.clone();

                    let bl3_file_type = match save.header_type {
                        HeaderType::PcSave => Bl3FileType::PcSave(save),
                        HeaderType::Ps4Save => Bl3FileType::Ps4Save(save),
                        _ => panic!("Unexpected Bl3FileType when reloading save"),
                    };

                    if loaded_file.filename() == save_file_name {
                        *loaded_file = bl3_file_type.clone();
                    } else {
                        self.loaded_files.push(bl3_file_type.clone());
                    }

                    self.loaded_files.sort();

                    self.loaded_files_selected = Box::new(bl3_file_type);

                    state_mappers::map_loaded_file_to_state(self);
                }
                MessageResult::Error(e) => {
                    let msg = format!("Failed to save file: {}", e);

                    self.notification =
                        Some(Notification::new(msg, NotificationSentiment::Negative));
                }
            },
            Message::SaveProfileCompleted(res) => match res {
                MessageResult::Success(profile) => {
                    self.notification = Some(Notification::new(
                        "Successfully saved profile!",
                        NotificationSentiment::Positive,
                    ));

                    let loaded_file_selected = &*self.loaded_files_selected;

                    let loaded_file = self
                        .loaded_files
                        .iter_mut()
                        .find(|f| *f == loaded_file_selected)
                        .expect("failed to find loaded file");

                    self.manage_profile_state.current_file = profile.clone();

                    let profile_file_name = profile.file_name.clone();

                    let bl3_file_type = match profile.header_type {
                        HeaderType::PcProfile => Bl3FileType::PcProfile(profile),
                        HeaderType::Ps4Profile => Bl3FileType::Ps4Profile(profile),
                        _ => panic!("Unexpected Bl3FileType when reloading profile"),
                    };

                    if loaded_file.filename() == profile_file_name {
                        *loaded_file = bl3_file_type.clone();
                    } else {
                        self.loaded_files.push(bl3_file_type.clone());
                    }

                    self.loaded_files.sort();

                    self.loaded_files_selected = Box::new(bl3_file_type);

                    state_mappers::map_loaded_file_to_state(self);
                }
                MessageResult::Error(e) => {
                    let msg = format!("Failed to save profile: {}", e);

                    self.notification =
                        Some(Notification::new(msg, NotificationSentiment::Negative));
                }
            },
            Message::ClearNotification => {
                self.notification = None;
            }
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

        let view_state_discrim = mem::discriminant(&self.view_state);
        let manage_save_discrim = mem::discriminant(&ViewState::ManageSave(
            ManageSaveView::TabBar(SaveTabBarView::General),
        ));
        let manage_profile_discrim = mem::discriminant(&ViewState::ManageProfile(
            ManageProfileView::TabBar(ProfileTabBarView::General),
        ));

        let mut save_button = Button::new(
            &mut self.save_file_button_state,
            Text::new("Save").font(JETBRAINS_MONO_BOLD).size(17),
        )
        .padding(10)
        .style(Bl3UiStyle);

        if view_state_discrim == manage_save_discrim {
            save_button = save_button.on_press(InteractionMessage::ManageSaveInteraction(
                ManageSaveInteractionMessage::SaveFilePressed,
            ));
        } else if view_state_discrim == manage_profile_discrim {
            save_button = save_button.on_press(InteractionMessage::ManageProfileInteraction(
                ManageProfileInteractionMessage::SaveProfilePressed,
            ));
        }

        let mut menu_bar_content = Row::new()
            .push(title)
            .spacing(15)
            .align_items(Align::Center);

        if view_state_discrim == manage_save_discrim || view_state_discrim == manage_profile_discrim
        {
            menu_bar_content = menu_bar_content.push(change_dir_button.into_element());
            menu_bar_content = menu_bar_content.push(all_saves_picklist);
            menu_bar_content = menu_bar_content.push(save_button.into_element());
        }

        let menu_bar = Container::new(menu_bar_content)
            .padding(20)
            .width(Length::Fill)
            .style(Bl3UiMenuBarStyle);

        let content = match &self.view_state {
            ViewState::Initializing => views::initialization::view(),
            ViewState::Loading => views::loading::view(),
            ViewState::ChooseSaveDirectory => {
                views::choose_save_directory::view(&mut self.choose_save_directory_state)
            }
            ViewState::ManageSave(manage_save_view) => match manage_save_view {
                ManageSaveView::TabBar(main_tab_bar_view) => {
                    views::manage_save::main::view(&mut self.manage_save_state, main_tab_bar_view)
                }
            },
            ViewState::ManageProfile(manage_profile_view) => match manage_profile_view {
                ManageProfileView::TabBar(main_tab_bar_view) => views::manage_profile::main::view(
                    &mut self.manage_profile_state,
                    main_tab_bar_view,
                ),
            },
        };

        let mut all_content = Column::new().push(menu_bar);

        if let Some(notification) = &mut self.notification {
            all_content = all_content.push(notification.view());
        }

        all_content = all_content.push(content);

        Container::new(all_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(Bl3UiContentStyle)
            .into()
    }
}
