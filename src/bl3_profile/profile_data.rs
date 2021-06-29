use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Error, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::bl3_profile::profile_currency::ProfileCurrency;
use crate::bl3_profile::science_levels::{BorderlandsScienceInfo, ScienceLevel};
use crate::bl3_profile::sdu::{ProfSduSlot, ProfSduSlotData};
use crate::bl3_profile::util::get_checksum_hash;
use crate::game_data::{
    PROFILE_ECHO_THEMES, PROFILE_ECHO_THEMES_DEFAULTS, PROFILE_EMOTES, PROFILE_EMOTES_DEFAULTS,
    PROFILE_HEADS, PROFILE_HEADS_DEFAULTS, PROFILE_SKINS, PROFILE_SKINS_DEFAULTS,
    PROFILE_WEAPON_SKINS, PROFILE_WEAPON_TRINKETS,
};
use crate::protos::oak_profile::Profile;

#[derive(Debug)]
pub struct ProfileData {
    pub profile: Profile,
    pub golden_keys: i32,
    pub diamond_keys: i32,
    pub vault_card_1_keys: i32,
    pub vault_card_1_chests: i32,
    pub guardian_rank: i32,
    pub guardian_rank_tokens: i32,
    pub borderlands_science_info: BorderlandsScienceInfo,
    pub sdu_slots: Vec<ProfSduSlotData>,
    pub bank_items: Vec<Vec<u8>>,
    pub lost_loot_items: Vec<Vec<u8>>,
    pub character_skins_unlocked: usize,
    pub character_heads_unlocked: usize,
    pub echo_themes_unlocked: usize,
    pub profile_emotes_unlocked: usize,
    pub room_decorations_unlocked: usize,
    pub weapon_skins_unlocked: usize,
    pub weapon_trinkets_unlocked: usize,
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

            let level = ScienceLevel::from_solves(borderlands_science_level_solves)
                .unwrap_or(ScienceLevel::Unknown);

            BorderlandsScienceInfo {
                science_level: level,
                solves,
                tokens: profile.CitizenScienceCSBucksAmount,
            }
        };

        let sdu_slots = profile
            .profile_sdu_list
            .par_iter()
            .map(|s| {
                let slot = ProfSduSlot::from_str(&s.sdu_data_path)?;
                let max = slot.maximum();

                Ok(ProfSduSlotData {
                    slot,
                    current: s.sdu_level,
                    max,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let bank_items = profile
            .bank_inventory_list
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        let lost_loot_items = profile
            .lost_loot_inventory_list
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        let mut character_skins_unlocked = PROFILE_SKINS_DEFAULTS.len();
        let mut character_heads_unlocked = PROFILE_HEADS_DEFAULTS.len();
        let mut echo_themes_unlocked = PROFILE_ECHO_THEMES_DEFAULTS.len();
        let mut profile_emotes_unlocked = PROFILE_EMOTES_DEFAULTS.len();

        profile.unlocked_customizations.iter().for_each(|uc| {
            let uc = uc.customization_asset_path.to_lowercase();
            let uc = &uc.as_str();

            match uc {
                uc if PROFILE_SKINS.contains(uc) => {
                    character_skins_unlocked += 1;
                }
                uc if PROFILE_HEADS.contains(uc) => {
                    character_heads_unlocked += 1;
                }
                uc if PROFILE_ECHO_THEMES.contains(uc) => {
                    echo_themes_unlocked += 1;
                }
                uc if PROFILE_EMOTES.contains(uc) => {
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
                            .map(|[k, _]| get_checksum_hash(k).unwrap_or(0))
                            .any(|hash| hash == uic_hash as usize) =>
                    {
                        weapon_skins_unlocked += 1;
                    }
                    uic_hash
                        if PROFILE_WEAPON_TRINKETS
                            .par_iter()
                            .map(|[k, _]| get_checksum_hash(k).unwrap_or(0))
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
            profile_emotes_unlocked,
            room_decorations_unlocked,
            weapon_skins_unlocked,
            weapon_trinkets_unlocked,
        })
    }
}
