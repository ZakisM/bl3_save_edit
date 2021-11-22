use std::convert::TryInto;

use anyhow::{Context, Result};
use derivative::Derivative;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum::{EnumMessage, IntoEnumIterator};
use tracing::error;

use crate::bl3_item::Bl3Item;
use crate::bl3_profile::guardian_reward::{GuardianReward, GuardianRewardData};
use crate::bl3_profile::profile_currency::ProfileCurrency;
use crate::bl3_profile::science_levels::{BorderlandsScienceInfo, BorderlandsScienceLevel};
use crate::bl3_profile::sdu::{ProfileSduSlot, ProfileSduSlotData};
use crate::bl3_profile::skins::{ProfileSkinType, SkinSet, WeaponSkinSet};
use crate::bl3_profile::util::get_checksum_hash;
use crate::game_data::{
    PROFILE_ECHO_THEMES, PROFILE_ECHO_THEMES_DEFAULTS, PROFILE_EMOTES, PROFILE_EMOTES_DEFAULTS,
    PROFILE_HEADS, PROFILE_HEADS_DEFAULTS, PROFILE_SKINS, PROFILE_SKINS_DEFAULTS,
    PROFILE_WEAPON_SKINS, PROFILE_WEAPON_TRINKETS,
};
use crate::protos::oak_profile::{
    GuardianRankProfileData, GuardianRankRewardSaveGameData, Profile,
};
use crate::protos::oak_shared::{
    CrewQuartersDecorationItemSaveGameData, InventoryCategorySaveData,
    OakCustomizationSaveGameData, OakInventoryCustomizationPartInfo, OakSDUSaveGameData,
    VaultCardRewardList, VaultCardSaveGameData,
};

#[derive(Derivative)]
#[derivative(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct ProfileData {
    #[derivative(Ord = "ignore", PartialOrd = "ignore")]
    pub profile: Profile,
    golden_keys: i32,
    diamond_keys: i32,
    vault_card_1_keys: i32,
    vault_card_1_chests: i32,
    vault_card_2_keys: i32,
    vault_card_2_chests: i32,
    vault_card_3_keys: i32,
    vault_card_3_chests: i32,
    guardian_rank: i32,
    guardian_tokens: i32,
    guardian_rewards: Vec<GuardianRewardData>,
    borderlands_science_info: BorderlandsScienceInfo,
    sdu_slots: Vec<ProfileSduSlotData>,
    bank_items: Vec<Bl3Item>,
    lost_loot_items: Vec<Bl3Item>,
    character_skins_unlocked: usize,
    character_heads_unlocked: usize,
    echo_themes_unlocked: usize,
    emotes_unlocked: usize,
    room_decorations_unlocked: usize,
    weapon_skins_unlocked: usize,
    weapon_trinkets_unlocked: usize,
}

impl ProfileData {
    pub fn from_profile(profile: Profile) -> Result<Self> {
        let golden_keys = ProfileCurrency::GoldenKey
            .get_profile_currency(&profile.bank_inventory_category_list)
            .unwrap_or(0);
        let diamond_keys = ProfileCurrency::DiamondKey
            .get_profile_currency(&profile.bank_inventory_category_list)
            .unwrap_or(0);
        let vault_card_1_keys = ProfileCurrency::VaultCardOneId
            .get_profile_currency(&profile.bank_inventory_category_list)
            .unwrap_or(0);

        let vault_card_1_chests = profile
            .vault_card
            .as_ref()
            .and_then(|vc| {
                vc.vault_card_claimed_rewards
                    .par_iter()
                    .find_first(|v| v.vault_card_id == 1)
                    .map(|v| v.vault_card_chests)
            })
            .unwrap_or(0);

        let vault_card_2_keys = ProfileCurrency::VaultCardTwoId
            .get_profile_currency(&profile.bank_inventory_category_list)
            .unwrap_or(0);

        let vault_card_2_chests = profile
            .vault_card
            .as_ref()
            .and_then(|vc| {
                vc.vault_card_claimed_rewards
                    .par_iter()
                    .find_first(|v| v.vault_card_id == 2)
                    .map(|v| v.vault_card_chests)
            })
            .unwrap_or(0);

        let vault_card_3_keys = ProfileCurrency::VaultCardThreeId
            .get_profile_currency(&profile.bank_inventory_category_list)
            .unwrap_or(0);

        let vault_card_3_chests = profile
            .vault_card
            .as_ref()
            .and_then(|vc| {
                vc.vault_card_claimed_rewards
                    .par_iter()
                    .find_first(|v| v.vault_card_id == 3)
                    .map(|v| v.vault_card_chests)
            })
            .unwrap_or(0);

        let guardian_rank_profile_data = profile
            .guardian_rank
            .as_ref()
            .context("failed to read profile Guardian Rank profile data.")?;

        let guardian_rank = guardian_rank_profile_data.guardian_rank;

        let guardian_rank_tokens = guardian_rank_profile_data.available_tokens;

        let guardian_rewards = GuardianReward::iter()
            .map(|reward| {
                let path = reward.get_serializations()[0];

                let current = guardian_rank_profile_data
                    .rank_rewards
                    .iter()
                    .find(|eg| eg.reward_data_path == path)
                    .map(|eg| eg.num_tokens)
                    .unwrap_or(0);

                GuardianRewardData {
                    current,
                    max: i32::MAX,
                    reward,
                }
            })
            .collect::<Vec<_>>();

        let borderlands_science_level_solves = &profile.CitizenScienceLevelProgression;

        let borderlands_science_info = {
            let solves = borderlands_science_level_solves.par_iter().sum();

            let level = BorderlandsScienceLevel::from_solves(borderlands_science_level_solves)
                .unwrap_or(BorderlandsScienceLevel::None);

            BorderlandsScienceInfo {
                science_level: level,
                solves,
                tokens: profile.CitizenScienceCSBucksAmount,
            }
        };

        let mut sdu_slots = ProfileSduSlot::iter()
            .map(|sdu| {
                let path = sdu.get_serializations()[0];

                let current = profile
                    .profile_sdu_list
                    .iter()
                    .find(|s| s.sdu_data_path == path)
                    .map(|s| s.sdu_level)
                    .unwrap_or(0);

                ProfileSduSlotData {
                    current,
                    max: sdu.maximum(),
                    sdu,
                }
            })
            .collect::<Vec<_>>();

        sdu_slots.sort();

        let bank_items = profile
            .bank_inventory_list
            .par_iter()
            .filter_map(|i| Bl3Item::from_serial_bytes(i, None).ok())
            .collect::<Vec<_>>();

        let lost_loot_items = profile
            .lost_loot_inventory_list
            .par_iter()
            .filter_map(|i| Bl3Item::from_serial_bytes(i, None).ok())
            .collect::<Vec<_>>();

        let mut character_skins_unlocked = PROFILE_SKINS_DEFAULTS.len();
        let mut character_heads_unlocked = PROFILE_HEADS_DEFAULTS.len();
        let mut echo_themes_unlocked = PROFILE_ECHO_THEMES_DEFAULTS.len();
        let mut profile_emotes_unlocked = PROFILE_EMOTES_DEFAULTS.len();

        profile.unlocked_customizations.iter().for_each(|uc| {
            let uc = &uc.customization_asset_path;
            let uc = &uc.as_str();

            match uc {
                uc if PROFILE_SKINS.iter().any(|gd| gd.ident == *uc) => {
                    character_skins_unlocked += 1;
                }
                uc if PROFILE_HEADS.iter().any(|gd| gd.ident == *uc) => {
                    character_heads_unlocked += 1;
                }
                uc if PROFILE_ECHO_THEMES.iter().any(|gd| gd.ident == *uc) => {
                    echo_themes_unlocked += 1;
                }
                uc if PROFILE_EMOTES.iter().any(|gd| gd.ident == *uc) => {
                    profile_emotes_unlocked += 1;
                }
                _ => (),
            };
        });

        let room_decorations_unlocked = profile.unlocked_crew_quarters_decorations.len();

        let mut weapon_skins_unlocked = 0;
        let mut weapon_trinkets_unlocked = 0;

        profile
            .unlocked_inventory_customization_parts
            .iter()
            .for_each(|uic| {
                let uic_hash = uic.customization_part_hash;

                match uic_hash {
                    uic_hash
                        if PROFILE_WEAPON_SKINS
                            .par_iter()
                            .map(|gd| get_checksum_hash(gd.ident).unwrap_or(0))
                            .any(|hash| hash == uic_hash as usize) =>
                    {
                        weapon_skins_unlocked += 1;
                    }
                    uic_hash
                        if PROFILE_WEAPON_TRINKETS
                            .par_iter()
                            .map(|gd| get_checksum_hash(gd.ident).unwrap_or(0))
                            .any(|hash| hash == uic_hash as usize) =>
                    {
                        weapon_trinkets_unlocked += 1;
                    }
                    _ => (),
                };
            });

        Ok(Self {
            profile,
            golden_keys,
            diamond_keys,
            vault_card_1_keys,
            vault_card_1_chests,
            vault_card_2_keys,
            vault_card_2_chests,
            vault_card_3_keys,
            vault_card_3_chests,
            guardian_rank,
            guardian_tokens: guardian_rank_tokens,
            guardian_rewards,
            borderlands_science_info,
            sdu_slots,
            bank_items,
            lost_loot_items,
            character_skins_unlocked,
            character_heads_unlocked,
            echo_themes_unlocked,
            emotes_unlocked: profile_emotes_unlocked,
            room_decorations_unlocked,
            weapon_skins_unlocked,
            weapon_trinkets_unlocked,
        })
    }

    pub fn golden_keys(&self) -> i32 {
        self.golden_keys
    }

    pub fn diamond_keys(&self) -> i32 {
        self.diamond_keys
    }

    pub fn vault_card_1_keys(&self) -> i32 {
        self.vault_card_1_keys
    }

    pub fn vault_card_1_chests(&self) -> i32 {
        self.vault_card_1_chests
    }

    pub fn vault_card_2_keys(&self) -> i32 {
        self.vault_card_2_keys
    }

    pub fn vault_card_2_chests(&self) -> i32 {
        self.vault_card_2_chests
    }

    pub fn vault_card_3_keys(&self) -> i32 {
        self.vault_card_3_keys
    }

    pub fn vault_card_3_chests(&self) -> i32 {
        self.vault_card_3_chests
    }

    pub fn set_currency(&mut self, currency: &ProfileCurrency, quantity: i32) -> Result<()> {
        let hash = currency
            .get_hash()
            .and_then(|h| h.try_into().map_err(anyhow::Error::new))
            .with_context(|| format!("failed to read hash for currency: {}", currency))?;

        if let Some(inv_cat_save_data) = self
            .profile
            .bank_inventory_category_list
            .iter_mut()
            .find(|i| i.base_category_definition_hash == hash)
        {
            inv_cat_save_data.quantity = quantity;
        } else {
            self.profile
                .bank_inventory_category_list
                .push(InventoryCategorySaveData {
                    base_category_definition_hash: hash,
                    quantity,
                    unknown_fields: Default::default(),
                    cached_size: Default::default(),
                });
        }

        match currency {
            ProfileCurrency::GoldenKey => {
                self.golden_keys = quantity;
            }
            ProfileCurrency::DiamondKey => {
                self.diamond_keys = quantity;
            }
            ProfileCurrency::VaultCardOneId => {
                self.vault_card_1_keys = quantity;
            }
            ProfileCurrency::VaultCardTwoId => {
                self.vault_card_2_keys = quantity;
            }
            ProfileCurrency::VaultCardThreeId => {
                self.vault_card_3_keys = quantity;
            }
        }

        Ok(())
    }

    pub fn set_vault_card_chests(&mut self, vault_card_id: u32, vault_card_chests: i32) {
        let vault_card_reward_list = VaultCardRewardList {
            vault_card_id,
            vault_card_experience: 0,
            unlocked_reward_list: Default::default(),
            redeemed_reward_list: Default::default(),
            vault_card_chests,
            vault_card_chests_opened: 0,
            vault_card_keys_spent: 0,
            gear_rewards: Default::default(),
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        };

        if let Some(vault_card) = self.profile.vault_card.as_mut() {
            if vault_card.last_active_vault_card_id == 0 {
                vault_card.last_active_vault_card_id = vault_card_id;
            }

            if let Some(claimed_rewards) = vault_card
                .vault_card_claimed_rewards
                .iter_mut()
                .find(|v| v.vault_card_id == vault_card_id)
            {
                claimed_rewards.vault_card_chests = vault_card_chests;
            } else {
                vault_card
                    .vault_card_claimed_rewards
                    .push(vault_card_reward_list);
            }
        } else {
            self.profile.vault_card = Some(VaultCardSaveGameData {
                last_active_vault_card_id: vault_card_id,
                current_day_seed: 0,
                current_week_seed: 0,
                vault_card_previous_challenges: Default::default(),
                vault_card_claimed_rewards: vec![vault_card_reward_list].into(),
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            })
            .into();
        }

        match vault_card_id {
            1 => self.vault_card_1_chests = vault_card_chests,
            2 => self.vault_card_2_chests = vault_card_chests,
            3 => self.vault_card_3_chests = vault_card_chests,
            _ => (),
        }
    }

    pub fn guardian_rank(&self) -> i32 {
        self.guardian_rank
    }

    pub fn set_guardian_rank(&mut self, new_rank: i32, tokens: Option<i32>) {
        if let Some(guardian_rank) = self.profile.guardian_rank.as_mut() {
            guardian_rank.guardian_rank = new_rank;

            if let Some(tokens) = tokens {
                guardian_rank.available_tokens = tokens;
            }
        } else {
            let guardian_rank = GuardianRankProfileData {
                available_tokens: tokens.unwrap_or(0),
                rank_rewards: Default::default(),
                guardian_rank: new_rank,
                guardian_experience: 0,
                guardian_reward_random_seed: 0,
                new_guardian_experience: 0,
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            };

            self.profile.guardian_rank = Some(guardian_rank).into();
        }

        self.guardian_rank = new_rank;

        if let Some(tokens) = tokens {
            self.guardian_tokens = tokens;
        }
    }

    pub fn guardian_tokens(&self) -> i32 {
        self.guardian_tokens
    }

    pub fn guardian_rewards(&self) -> &Vec<GuardianRewardData> {
        &self.guardian_rewards
    }

    pub fn set_guardian_reward(
        &mut self,
        guardian_reward: &GuardianReward,
        tokens: i32,
    ) -> Result<()> {
        let reward_path = guardian_reward.get_serializations()[0];

        let guardian_rank_profile_data = self
            .profile
            .guardian_rank
            .as_mut()
            .context("failed to read profile Guardian Rank profile data.")?;

        if let Some(reward) = guardian_rank_profile_data
            .rank_rewards
            .iter_mut()
            .find(|eg| eg.reward_data_path == reward_path)
        {
            reward.num_tokens = tokens;
        } else {
            guardian_rank_profile_data
                .rank_rewards
                .push(GuardianRankRewardSaveGameData {
                    num_tokens: tokens,
                    reward_data_path: reward_path.to_owned(),
                    unknown_fields: Default::default(),
                    cached_size: Default::default(),
                });
        }

        if let Some(existing) = self
            .guardian_rewards
            .iter_mut()
            .find(|g| &g.reward == guardian_reward)
        {
            existing.current = tokens;
        }

        Ok(())
    }

    pub fn borderlands_science_info(&self) -> &BorderlandsScienceInfo {
        &self.borderlands_science_info
    }

    pub fn set_borderlands_science_level(&mut self, science_level: &BorderlandsScienceLevel) {
        self.profile.CitizenScienceLevelProgression = science_level.progression();
        self.profile.bCitizenScienceHasSeenIntroVideo = true;
        self.profile.bCitizenScienceTutorialDone = true;

        self.borderlands_science_info.science_level = science_level.to_owned();
    }

    pub fn set_borderlands_science_tokens(&mut self, tokens: i32) {
        self.profile.CitizenScienceCSBucksAmount = tokens;

        self.borderlands_science_info.tokens = tokens;
    }

    pub fn sdu_slots(&self) -> &Vec<ProfileSduSlotData> {
        &self.sdu_slots
    }

    pub fn set_sdu_slot(&mut self, sdu_slot: &ProfileSduSlot, level: i32) {
        let sdu_path = sdu_slot.get_serializations()[0];

        if let Some(sdu) = self
            .profile
            .profile_sdu_list
            .iter_mut()
            .find(|s| s.sdu_data_path == sdu_path)
        {
            sdu.sdu_level = level;
        } else {
            self.profile.profile_sdu_list.push(OakSDUSaveGameData {
                sdu_level: level,
                sdu_data_path: sdu_path.to_string(),
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            })
        }

        if let Some(current_slot) = self.sdu_slots.iter_mut().find(|i| i.sdu == *sdu_slot) {
            current_slot.current = level;
        } else {
            self.sdu_slots.push(ProfileSduSlotData {
                sdu: sdu_slot.to_owned(),
                current: level,
                max: sdu_slot.maximum(),
            });
        }
    }

    pub fn bank_items(&self) -> &Vec<Bl3Item> {
        &self.bank_items
    }

    pub fn bank_items_mut(&mut self) -> &mut Vec<Bl3Item> {
        &mut self.bank_items
    }

    pub fn remove_bank_item(&mut self, index: usize) {
        if index < self.profile.bank_inventory_list.len() {
            self.profile.bank_inventory_list.remove(index);
        }

        if index < self.bank_items.len() {
            self.bank_items.remove(index);
        }
    }

    pub fn add_bank_item(&mut self, item: &Bl3Item) -> Result<()> {
        let item_serial_number = item.get_serial_number(true)?;

        self.profile.bank_inventory_list.push(item_serial_number);

        self.bank_items.push(item.to_owned());

        Ok(())
    }

    pub fn insert_bank_item(&mut self, item_index: usize, item: &Bl3Item) -> Result<()> {
        let item_serial_number = item.get_serial_number(true)?;

        self.profile
            .bank_inventory_list
            .insert(item_index, item_serial_number);

        self.bank_items.insert(item_index, item.to_owned());

        Ok(())
    }

    pub fn replace_bank_item(&mut self, item_index: usize, new_item: &Bl3Item) -> Result<()> {
        self.insert_bank_item(item_index, new_item)?;

        // Remove old item
        self.remove_bank_item(item_index + 1);

        Ok(())
    }

    pub fn lost_loot_items(&self) -> &Vec<Bl3Item> {
        &self.lost_loot_items
    }

    pub fn character_skins_unlocked(&self) -> usize {
        self.character_skins_unlocked
    }

    pub fn character_heads_unlocked(&self) -> usize {
        self.character_heads_unlocked
    }

    pub fn echo_themes_unlocked(&self) -> usize {
        self.echo_themes_unlocked
    }

    pub fn profile_emotes_unlocked(&self) -> usize {
        self.emotes_unlocked
    }

    pub fn room_decorations_unlocked(&self) -> usize {
        self.room_decorations_unlocked
    }

    pub fn weapon_skins_unlocked(&self) -> usize {
        self.weapon_skins_unlocked
    }

    pub fn weapon_trinkets_unlocked(&self) -> usize {
        self.weapon_trinkets_unlocked
    }

    pub fn unlock_skin_set(&mut self, skin_type: &ProfileSkinType) {
        let mut skins = skin_type.skin_set();

        skins.sort_by_key(|s| s.name);

        match skin_type {
            ProfileSkinType::Regular(set) => match set {
                SkinSet::RoomDecorations => {
                    // Use previous customizations as we want to re-order alphabetically
                    let previous_customizations =
                        self.profile.unlocked_crew_quarters_decorations.clone();

                    self.profile.unlocked_crew_quarters_decorations.clear();

                    skins.iter().for_each(|c| {
                        self.profile.unlocked_crew_quarters_decorations.push(
                            CrewQuartersDecorationItemSaveGameData {
                                is_new: true,
                                decoration_item_asset_path: c.ident.to_owned(),
                                unknown_fields: Default::default(),
                                cached_size: Default::default(),
                            },
                        );
                    });

                    previous_customizations.iter().for_each(|pc| {
                        if !self
                            .profile
                            .unlocked_crew_quarters_decorations
                            .iter()
                            .any(|ucd| {
                                ucd.decoration_item_asset_path
                                    .eq_ignore_ascii_case(&pc.decoration_item_asset_path)
                            })
                        {
                            self.profile
                                .unlocked_crew_quarters_decorations
                                .push(pc.to_owned());
                        }
                    });
                }
                _ => {
                    skins.iter().for_each(|c| {
                        if !self
                            .profile
                            .unlocked_customizations
                            .iter()
                            .any(|uc| uc.customization_asset_path.eq_ignore_ascii_case(c.ident))
                        {
                            self.profile.unlocked_customizations.push(
                                OakCustomizationSaveGameData {
                                    is_new: true,
                                    customization_asset_path: c.ident.to_owned(),
                                    unknown_fields: Default::default(),
                                    cached_size: Default::default(),
                                },
                            );
                        }
                    });
                }
            },
            ProfileSkinType::Weapon(_) => {
                skins.iter().for_each(|c| {
                    if let Ok(hash) = get_checksum_hash(c.ident)
                        .and_then(|h| h.try_into().map_err(anyhow::Error::new))
                    {
                        let contains = self
                            .profile
                            .unlocked_inventory_customization_parts
                            .iter()
                            .any(|uic| uic.customization_part_hash == hash);

                        if !contains {
                            self.profile.unlocked_inventory_customization_parts.push(
                                OakInventoryCustomizationPartInfo {
                                    customization_part_hash: hash,
                                    is_new: true,
                                    unknown_fields: Default::default(),
                                    cached_size: Default::default(),
                                },
                            );
                        }
                    } else {
                        error!("When trying to unlock Weapon Skin/Trinket, failed to get hash for: {}.", c.ident);
                    }
                });
            }
        };

        match skin_type {
            ProfileSkinType::Regular(r) => match r {
                SkinSet::CharacterSkins => self.character_skins_unlocked = skin_type.maximum(),
                SkinSet::CharacterHeads => self.character_heads_unlocked = skin_type.maximum(),
                SkinSet::EchoThemes => self.echo_themes_unlocked = skin_type.maximum(),
                SkinSet::Emotes => self.emotes_unlocked = skin_type.maximum(),
                SkinSet::RoomDecorations => self.room_decorations_unlocked = skin_type.maximum(),
            },
            ProfileSkinType::Weapon(w) => match w {
                WeaponSkinSet::WeaponSkins => self.weapon_skins_unlocked = skin_type.maximum(),
                WeaponSkinSet::WeaponTrinkets => {
                    self.weapon_trinkets_unlocked = skin_type.maximum()
                }
            },
        }
    }
}
