use std::convert::TryInto;
use std::str::FromStr;

use anyhow::Result;
use derivative::Derivative;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use strum::{EnumMessage, IntoEnumIterator};

use crate::bl3_profile::profile_currency::ProfileCurrency;
use crate::bl3_profile::science_levels::{BorderlandsScienceInfo, BorderlandsScienceLevel};
use crate::bl3_profile::sdu::{ProfileSduSlot, ProfileSduSlotData};
use crate::bl3_profile::skins::{ProfileSkinType, SkinSet, WeaponSkinSet};
use crate::bl3_profile::util::get_checksum_hash;
use crate::bl3_save::bl3_item::Bl3Item;
use crate::game_data::{
    PROFILE_ECHO_THEMES, PROFILE_ECHO_THEMES_DEFAULTS, PROFILE_EMOTES, PROFILE_EMOTES_DEFAULTS,
    PROFILE_HEADS, PROFILE_HEADS_DEFAULTS, PROFILE_SKINS, PROFILE_SKINS_DEFAULTS,
    PROFILE_WEAPON_SKINS, PROFILE_WEAPON_TRINKETS,
};
use crate::protos::oak_profile::{GuardianRankProfileData, Profile};
use crate::protos::oak_shared::{
    OakCustomizationSaveGameData, OakInventoryCustomizationPartInfo, OakSDUSaveGameData,
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
    guardian_rank: i32,
    guardian_rank_tokens: i32,
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

        let guardian_rank = profile
            .guardian_rank
            .as_ref()
            .map(|g| g.guardian_rank)
            .unwrap_or(0);

        let guardian_rank_tokens = profile
            .guardian_rank
            .as_ref()
            .map(|g| g.available_tokens)
            .unwrap_or(0);

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

        let mut sdu_slots = profile
            .profile_sdu_list
            .par_iter()
            .map(|s| {
                let slot = ProfileSduSlot::from_str(&s.sdu_data_path)?;
                let max = slot.maximum();

                Ok(ProfileSduSlotData {
                    slot,
                    current: s.sdu_level,
                    max,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        // make sure that we include all sdu slots that might not be in our save
        ProfileSduSlot::iter().for_each(|sdu| {
            let contains_sdu_slot = sdu_slots.par_iter().any(|profile_sdu| {
                std::mem::discriminant(&sdu) == std::mem::discriminant(&profile_sdu.slot)
            });

            if !contains_sdu_slot {
                sdu_slots.push(ProfileSduSlotData {
                    current: 0,
                    max: sdu.maximum(),
                    slot: sdu,
                })
            }
        });

        sdu_slots.par_sort();

        let bank_items = profile
            .bank_inventory_list
            .par_iter()
            .filter_map(|i| Bl3Item::from_serial_bytes(i).ok())
            .collect::<Vec<_>>();

        let lost_loot_items = profile
            .lost_loot_inventory_list
            .par_iter()
            .filter_map(|i| Bl3Item::from_serial_bytes(i).ok())
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
            guardian_rank,
            guardian_rank_tokens,
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
            self.guardian_rank_tokens = tokens;
        }
    }

    pub fn guardian_rank_tokens(&self) -> i32 {
        self.guardian_rank_tokens
    }

    pub fn borderlands_science_info(&self) -> &BorderlandsScienceInfo {
        &self.borderlands_science_info
    }

    pub fn set_borderlands_science_level(&mut self, science_level: &BorderlandsScienceLevel) {
        self.profile.CitizenScienceLevelProgression = science_level.science_level_progression();
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

        if let Some(current_slot) = self.sdu_slots.iter_mut().find(|i| i.slot == *sdu_slot) {
            current_slot.current = level;
        } else {
            self.sdu_slots.push(ProfileSduSlotData {
                slot: sdu_slot.to_owned(),
                current: level,
                max: sdu_slot.maximum(),
            });
        }
    }

    pub fn bank_items(&self) -> &Vec<Bl3Item> {
        &self.bank_items
    }

    pub fn remove_bank_item(&mut self, index: usize) {
        if index < self.profile.bank_inventory_list.len() {
            self.profile.bank_inventory_list.remove(index);
        }

        if index < self.bank_items.len() {
            self.bank_items.remove(index);
        }
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
            ProfileSkinType::Regular(_) => {
                let previous_customizations = self.profile.unlocked_customizations.clone();

                self.profile.unlocked_customizations.clear();

                skins.iter().for_each(|c| {
                    self.profile
                        .unlocked_customizations
                        .push(OakCustomizationSaveGameData {
                            is_new: true,
                            customization_asset_path: c.ident.to_owned(),
                            unknown_fields: Default::default(),
                            cached_size: Default::default(),
                        });
                });

                previous_customizations.iter().for_each(|pc| {
                    if !self.profile.unlocked_customizations.contains(pc) {
                        self.profile.unlocked_customizations.push(pc.to_owned());
                    }
                });
            }
            ProfileSkinType::Weapon(_) => {
                let previous_inventory_customizations =
                    self.profile.unlocked_inventory_customization_parts.clone();

                self.profile.unlocked_inventory_customization_parts.clear();

                skins.iter().for_each(|c| {
                    let hash: u32 = get_checksum_hash(c.ident)
                        .and_then(|h| h.try_into().map_err(anyhow::Error::new))
                        .unwrap_or(0);

                    self.profile.unlocked_inventory_customization_parts.push(
                        OakInventoryCustomizationPartInfo {
                            customization_part_hash: hash,
                            is_new: true,
                            unknown_fields: Default::default(),
                            cached_size: Default::default(),
                        },
                    );
                });

                previous_inventory_customizations.iter().for_each(|pc| {
                    if !self
                        .profile
                        .unlocked_inventory_customization_parts
                        .contains(pc)
                    {
                        self.profile
                            .unlocked_inventory_customization_parts
                            .push(pc.to_owned());
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
