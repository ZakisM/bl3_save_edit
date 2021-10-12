use std::str::FromStr;

use anyhow::{Context, Result};
use derivative::Derivative;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum::{EnumMessage, IntoEnumIterator};

use crate::bl3_item::{Bl3Item, ItemFlags};
use crate::bl3_profile::guardian_reward::GuardianReward;
use crate::bl3_save::ammo::{AmmoPool, AmmoPoolData};
use crate::bl3_save::challenge_data::Challenge;
use crate::bl3_save::challenge_data::ChallengeData;
use crate::bl3_save::inventory_slot::{InventorySlot, InventorySlotData};
use crate::bl3_save::level_data::{LEVEL_CHALLENGES, LEVEL_STAT};
use crate::bl3_save::models::Currency;
use crate::bl3_save::player_class::PlayerClass;
use crate::bl3_save::playthrough::Playthrough;
use crate::bl3_save::sdu::{SaveSduSlot, SaveSduSlotData};
use crate::bl3_save::util::{currency_amount_from_character, experience_to_level};
use crate::game_data::{
    GameDataKv, PROFILE_ECHO_THEMES, PROFILE_ECHO_THEMES_DEFAULTS, PROFILE_HEADS,
    PROFILE_HEADS_DEFAULTS, PROFILE_SKINS, PROFILE_SKINS_DEFAULTS, VEHICLE_CHASSIS_CYCLONE,
    VEHICLE_CHASSIS_JETBEAST, VEHICLE_CHASSIS_OUTRUNNER, VEHICLE_CHASSIS_TECHNICAL,
    VEHICLE_PARTS_CYCLONE, VEHICLE_PARTS_JETBEAST, VEHICLE_PARTS_OUTRUNNER,
    VEHICLE_PARTS_TECHNICAL, VEHICLE_SKINS_CYCLONE, VEHICLE_SKINS_JETBEAST,
    VEHICLE_SKINS_OUTRUNNER, VEHICLE_SKINS_TECHNICAL,
};
use crate::protos::oak_save::{
    Character, GuardianRankCharacterSaveGameData, GuardianRankRewardCharacterSaveGameData,
    GuardianRankSaveGameData, OakInventoryItemSaveGameData, VehicleUnlockedSaveGameData,
};
use crate::protos::oak_shared::{
    GameStatSaveGameData, InventoryCategorySaveData, OakSDUSaveGameData,
};
use crate::vehicle_data::{VehicleData, VehicleSubType, VehicleType};

pub const MAX_CHARACTER_LEVEL: usize = 72;

#[derive(Derivative)]
#[derivative(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct CharacterData {
    #[derivative(Ord = "ignore", PartialOrd = "ignore")]
    pub character: Character,
    player_class: PlayerClass,
    player_level: i32,
    ability_points: i32,
    guardian_rank: i32,
    head_skin_selected: GameDataKv,
    character_skin_selected: GameDataKv,
    echo_theme_selected: GameDataKv,
    money: i32,
    eridium: i32,
    playthroughs: Vec<Playthrough>,
    unlockable_inventory_slots: Vec<InventorySlotData>,
    sdu_slots: Vec<SaveSduSlotData>,
    ammo_pools: Vec<AmmoPoolData>,
    challenge_milestones: Vec<ChallengeData>,
    vehicle_data: [VehicleData; 12],
    inventory_items: Vec<Bl3Item>,
}

impl CharacterData {
    pub fn from_character(character: Character) -> Result<Self> {
        let player_class = PlayerClass::from_str(
            character
                .player_class_data
                .as_ref()
                .map(|p| p.player_class_path.as_str())
                .context("failed to read player class")?,
        )?;

        let player_level = experience_to_level(character.experience_points)?;

        let ability_points = character
            .ability_data
            .as_ref()
            .context("failed to read Player ability data")?
            .ability_points;

        let guardian_rank = character
            .guardian_rank_character_data
            .as_ref()
            .map(|g| g.guardian_rank)
            .unwrap_or(0);

        let available_head_skins = PROFILE_HEADS_DEFAULTS
            .par_iter()
            .chain(PROFILE_HEADS.par_iter())
            .filter(|h| h.ident.contains(&player_class.to_string()))
            .cloned()
            .collect::<Vec<_>>();

        let head_skin_selected = available_head_skins
            .par_iter()
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs == s.ident)
            })
            .cloned()
            .unwrap_or(available_head_skins[0]);

        let available_character_skins = PROFILE_SKINS_DEFAULTS
            .par_iter()
            .chain(PROFILE_SKINS.par_iter())
            .filter(|h| h.ident.contains(&player_class.to_string()))
            .cloned()
            .collect::<Vec<_>>();

        let character_skin_selected = available_character_skins
            .par_iter()
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs == s.ident)
            })
            .cloned()
            .unwrap_or(available_character_skins[0]);

        let echo_theme_selected = PROFILE_ECHO_THEMES_DEFAULTS
            .par_iter()
            .chain(PROFILE_ECHO_THEMES.par_iter())
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs == s.ident)
            })
            .map(|s| s.to_owned())
            .unwrap_or(PROFILE_ECHO_THEMES_DEFAULTS[0]);

        let money = currency_amount_from_character(&character, &Currency::Money);
        let eridium = currency_amount_from_character(&character, &Currency::Eridium);
        let playthroughs = Playthrough::playthroughs_from_character(&character)?;

        let mut unlockable_inventory_slots = character
            .equipped_inventory_list
            .par_iter()
            .map(|i| {
                Ok(InventorySlotData {
                    slot: InventorySlot::from_str(&i.slot_data_path).with_context(|| {
                        format!("failed to read inventory slot: {}", &i.slot_data_path)
                    })?,
                    unlocked: i.enabled,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        unlockable_inventory_slots.sort();

        let mut sdu_slots = SaveSduSlot::iter()
            .map(|sdu| {
                let path = sdu.get_serializations()[0];

                let current = character
                    .sdu_list
                    .iter()
                    .find(|s| s.sdu_data_path == path)
                    .map(|s| s.sdu_level)
                    .unwrap_or(0);

                SaveSduSlotData {
                    current,
                    max: sdu.maximum(),
                    sdu,
                }
            })
            .collect::<Vec<_>>();

        sdu_slots.sort();

        let mut ammo_pools = character
            .resource_pools
            .iter()
            .filter(|rp| !rp.resource_path.contains("Eridium"))
            .map(|rp| {
                let ammo = AmmoPool::from_str(&rp.resource_path)
                    .with_context(|| format!("failed to read ammo: {}", &rp.resource_path))?;

                let max = ammo.maximum();

                Ok(AmmoPoolData {
                    pool: ammo,
                    current: rp.amount as i32,
                    max,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        ammo_pools.sort();

        let mut challenge_milestones = Challenge::iter()
            .filter(|challenge| {
                let challenge_path = challenge.get_serializations()[0];

                if challenge_path.contains("Character") {
                    challenge_path.contains(&player_class.to_string())
                } else {
                    true
                }
            })
            .map(|challenge| {
                let chall_path = challenge.get_serializations()[0];

                let unlocked = character
                    .challenge_data
                    .par_iter()
                    .find_first(|cd| cd.challenge_class_path.contains(&chall_path))
                    .map(|cd| cd.currently_completed)
                    .context("failed to read challenge milestones")?;

                Ok(ChallengeData {
                    challenge,
                    unlocked,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        challenge_milestones.sort();

        let mut outrunner_chassis = 0;
        let mut jetbeast_chassis = 0;
        let mut technical_chassis = 0;
        let mut cyclone_chassis = 0;

        character.vehicles_unlocked_data.iter().for_each(|vu| {
            let vu = &vu.asset_path;
            let vu = &vu.as_str();

            match vu {
                vu if VEHICLE_CHASSIS_OUTRUNNER.contains(vu) => {
                    outrunner_chassis += 1;
                }
                vu if VEHICLE_CHASSIS_JETBEAST.contains(vu) => {
                    jetbeast_chassis += 1;
                }
                vu if VEHICLE_CHASSIS_TECHNICAL.contains(vu) => {
                    technical_chassis += 1;
                }
                vu if VEHICLE_CHASSIS_CYCLONE.contains(vu) => {
                    cyclone_chassis += 1;
                }
                _ => (),
            };
        });

        let mut outrunner_parts = 0;
        let mut jetbeast_parts = 0;
        let mut technical_parts = 0;
        let mut cyclone_parts = 0;

        let mut outrunner_skins = 0;
        let mut jetbeast_skins = 0;
        let mut technical_skins = 0;
        let mut cyclone_skins = 0;

        character.vehicle_parts_unlocked.iter().for_each(|vp| {
            let vp = vp;
            let vp = &vp.as_str();

            match vp {
                vp if VEHICLE_PARTS_OUTRUNNER.contains(vp) => {
                    outrunner_parts += 1;
                }
                vp if VEHICLE_PARTS_JETBEAST.contains(vp) => {
                    jetbeast_parts += 1;
                }
                vp if VEHICLE_PARTS_TECHNICAL.contains(vp) => {
                    technical_parts += 1;
                }
                vp if VEHICLE_PARTS_CYCLONE.contains(vp) => {
                    cyclone_parts += 1;
                }
                vp if VEHICLE_SKINS_OUTRUNNER.contains(vp) => {
                    outrunner_skins += 1;
                }
                vp if VEHICLE_SKINS_JETBEAST.contains(vp) => {
                    jetbeast_skins += 1;
                }
                vp if VEHICLE_SKINS_TECHNICAL.contains(vp) => {
                    technical_skins += 1;
                }
                vp if VEHICLE_SKINS_CYCLONE.contains(vp) => {
                    cyclone_skins += 1;
                }
                _ => (),
            };
        });

        let vehicle_data = [
            VehicleData {
                vehicle_type: VehicleType::Outrunner(VehicleSubType::Chassis),
                current: outrunner_chassis,
            },
            VehicleData {
                vehicle_type: VehicleType::Outrunner(VehicleSubType::Parts),
                current: outrunner_parts,
            },
            VehicleData {
                vehicle_type: VehicleType::Outrunner(VehicleSubType::Skins),
                current: outrunner_skins,
            },
            VehicleData {
                vehicle_type: VehicleType::Jetbeast(VehicleSubType::Chassis),
                current: jetbeast_chassis,
            },
            VehicleData {
                vehicle_type: VehicleType::Jetbeast(VehicleSubType::Parts),
                current: jetbeast_parts,
            },
            VehicleData {
                vehicle_type: VehicleType::Jetbeast(VehicleSubType::Skins),
                current: jetbeast_skins,
            },
            VehicleData {
                vehicle_type: VehicleType::Technical(VehicleSubType::Chassis),
                current: technical_chassis,
            },
            VehicleData {
                vehicle_type: VehicleType::Technical(VehicleSubType::Parts),
                current: technical_parts,
            },
            VehicleData {
                vehicle_type: VehicleType::Technical(VehicleSubType::Skins),
                current: technical_skins,
            },
            VehicleData {
                vehicle_type: VehicleType::Cyclone(VehicleSubType::Chassis),
                current: cyclone_chassis,
            },
            VehicleData {
                vehicle_type: VehicleType::Cyclone(VehicleSubType::Parts),
                current: cyclone_parts,
            },
            VehicleData {
                vehicle_type: VehicleType::Cyclone(VehicleSubType::Skins),
                current: cyclone_skins,
            },
        ];

        let inventory_items = character
            .inventory_items
            .par_iter()
            .filter_map(|i| {
                Bl3Item::from_serial_bytes(&i.item_serial_number, ItemFlags::from_bits(i.flags))
                    .ok()
            })
            .collect::<Vec<_>>();

        Ok(Self {
            character,
            player_class,
            player_level,
            ability_points,
            guardian_rank,
            head_skin_selected,
            character_skin_selected,
            echo_theme_selected,
            money,
            eridium,
            playthroughs,
            unlockable_inventory_slots,
            sdu_slots,
            ammo_pools,
            challenge_milestones,
            vehicle_data,
            inventory_items,
        })
    }

    pub fn player_class(&self) -> PlayerClass {
        self.player_class
    }

    pub fn set_player_class(&mut self, player_class: PlayerClass) -> Result<()> {
        if player_class != self.player_class {
            let player_class_data = self
                .character
                .player_class_data
                .as_mut()
                .with_context(|| "failed to read Player Class data")?;

            player_class_data.player_class_path = player_class.get_serializations()[0].to_string();

            let ability_data = self
                .character
                .ability_data
                .as_mut()
                .context("failed to read Player ability data")?;

            //Reset our skill tree also
            ability_data
                .tree_item_list
                .iter_mut()
                .for_each(|ti| ti.points = 0);

            if self.player_level > 2 {
                let new_ability_points = self.player_level - 2;

                ability_data.ability_points = new_ability_points;

                self.ability_points = new_ability_points;
            }

            self.player_class = player_class;
        }

        Ok(())
    }

    pub fn player_level(&self) -> i32 {
        self.player_level
    }

    pub fn set_player_level(&mut self, experience_points: i32) -> Result<()> {
        if experience_points != self.character.experience_points {
            self.player_level = experience_to_level(experience_points).with_context(|| {
                format!(
                    "failed to set level for experience points: {}",
                    experience_points
                )
            })?;

            self.character.experience_points = experience_points;

            self.set_game_stat(LEVEL_STAT, self.player_level);

            let ability_data = self
                .character
                .ability_data
                .as_mut()
                .context("failed to read Player ability data")?;

            //Reset existing skill tree
            ability_data
                .tree_item_list
                .iter_mut()
                .for_each(|ti| ti.points = 0);

            //Unlock skill tree
            if self.player_level > 1 && ability_data.tree_grade == 0 {
                ability_data.tree_grade = 2;
            }

            if self.player_level > 2 {
                let new_ability_points = self.player_level - 2;

                ability_data.ability_points = new_ability_points;

                self.ability_points = new_ability_points;
            }

            for (challenge_level, challenge_obj) in LEVEL_CHALLENGES {
                if self.player_level >= challenge_level {
                    self.unlock_challenge_obj(challenge_obj, 1, 0)?;
                }
            }
        }

        Ok(())
    }

    pub fn ability_points(&self) -> i32 {
        self.ability_points
    }

    pub fn set_ability_points(&mut self, ability_points: i32) -> Result<()> {
        let ability_data = self
            .character
            .ability_data
            .as_mut()
            .context("failed to read Player ability data")?;

        if ability_points != ability_data.ability_points {
            ability_data.ability_points = ability_points;

            self.ability_points = ability_points;
        }

        Ok(())
    }

    pub fn guardian_rank(&self) -> i32 {
        self.guardian_rank
    }

    pub fn set_guardian_rank(&mut self, new_rank: i32, tokens: Option<i32>) {
        if let Some(guardian_rank) = self.character.guardian_rank.as_mut() {
            guardian_rank.guardian_rank = new_rank;
        } else {
            let guardian_rank = GuardianRankSaveGameData {
                guardian_rank: new_rank,
                guardian_experience: 0,
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            };

            self.character.guardian_rank = Some(guardian_rank).into();
        }

        if let Some(guardian_data) = self.character.guardian_rank_character_data.as_mut() {
            guardian_data.guardian_rank = new_rank;

            if let Some(tokens) = tokens {
                guardian_data.guardian_available_tokens = tokens;
            }
        } else {
            let guardian_data = GuardianRankCharacterSaveGameData {
                guardian_available_tokens: tokens.unwrap_or(0),
                guardian_rank: new_rank,
                guardian_experience: 0,
                rank_rewards: Default::default(),
                rank_perks: Default::default(),
                guardian_reward_random_seed: 0,
                new_guardian_experience: 0,
                is_rank_system_enabled: false,
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            };

            self.character.guardian_rank_character_data = Some(guardian_data).into();
        }

        self.guardian_rank = new_rank;
    }

    pub fn set_guardian_reward(
        &mut self,
        guardian_reward: &GuardianReward,
        tokens: i32,
    ) -> Result<()> {
        let reward_path = guardian_reward.get_serializations()[0];

        let guardian_rank_character_data = self
            .character
            .guardian_rank_character_data
            .as_mut()
            .context("failed to read character Guardian Rank character data.")?;

        if let Some(reward) = guardian_rank_character_data
            .rank_rewards
            .iter_mut()
            .find(|eg| eg.reward_data_path == reward_path)
        {
            reward.num_tokens = tokens;
        } else {
            guardian_rank_character_data.rank_rewards.push(
                GuardianRankRewardCharacterSaveGameData {
                    num_tokens: tokens,
                    is_enabled: true,
                    reward_data_path: reward_path.to_owned(),
                    unknown_fields: Default::default(),
                    cached_size: Default::default(),
                },
            );
        }

        Ok(())
    }

    pub fn head_skin_selected(&self) -> GameDataKv {
        self.head_skin_selected
    }

    pub fn set_head_skin_selected(&mut self, head_skin_selected: &GameDataKv) {
        let curr_head_skin_selected = self.head_skin_selected.ident;
        let head_skin_selected_id = head_skin_selected.ident.to_owned();

        if let Some(curr_head) = self
            .character
            .selected_customizations
            .iter_mut()
            .find(|curr| curr_head_skin_selected == *curr)
        {
            *curr_head = head_skin_selected_id;
        } else {
            self.character
                .selected_customizations
                .push(head_skin_selected_id)
        }

        self.head_skin_selected = head_skin_selected.to_owned();
    }

    pub fn character_skin_selected(&self) -> GameDataKv {
        self.character_skin_selected
    }

    pub fn set_character_skin_selected(&mut self, character_skin_selected: &GameDataKv) {
        let curr_character_skin_selected = self.character_skin_selected.ident;
        let character_skin_selected_id = character_skin_selected.ident.to_owned();

        if let Some(curr_skin) = self
            .character
            .selected_customizations
            .iter_mut()
            .find(|curr| curr_character_skin_selected == *curr)
        {
            *curr_skin = character_skin_selected_id;
        } else {
            self.character
                .selected_customizations
                .push(character_skin_selected_id)
        }

        self.character_skin_selected = character_skin_selected.to_owned();
    }

    pub fn echo_theme_selected(&self) -> GameDataKv {
        self.echo_theme_selected
    }

    pub fn set_echo_theme_selected(&mut self, echo_theme_selected: &GameDataKv) {
        let curr_echo_theme_selected = self.echo_theme_selected.ident;
        let echo_theme_selected_id = echo_theme_selected.ident.to_owned();

        if let Some(echo_theme) = self
            .character
            .selected_customizations
            .iter_mut()
            .find(|curr| curr_echo_theme_selected == *curr)
        {
            *echo_theme = echo_theme_selected_id;
        } else {
            self.character
                .selected_customizations
                .push(echo_theme_selected_id)
        }

        self.echo_theme_selected = echo_theme_selected.to_owned();
    }

    pub fn money(&self) -> i32 {
        self.money
    }

    pub fn set_money(&mut self, amount: i32) -> Result<()> {
        self.money = amount;

        if let Some(money) = self
            .character
            .inventory_category_list
            .iter_mut()
            .find(|i| i.base_category_definition_hash == Currency::Money.hash_value())
        {
            money.quantity = amount;
        } else {
            self.add_inventory_category_item(Currency::Money.hash_value(), amount);
        }

        Ok(())
    }

    pub fn eridium(&self) -> i32 {
        self.eridium
    }

    pub fn set_eridium(&mut self, amount: i32) -> Result<()> {
        self.eridium = amount;

        if let Some(eridium) = self
            .character
            .inventory_category_list
            .iter_mut()
            .find(|i| i.base_category_definition_hash == Currency::Eridium.hash_value())
        {
            eridium.quantity = amount;
        } else {
            self.add_inventory_category_item(Currency::Eridium.hash_value(), amount);
        }

        Ok(())
    }

    pub fn playthroughs(&self) -> &Vec<Playthrough> {
        &self.playthroughs
    }

    pub fn unlockable_inventory_slots(&self) -> &Vec<InventorySlotData> {
        &self.unlockable_inventory_slots
    }

    pub fn remove_inventory_slot_if_exists(
        &mut self,
        inventory_slot: &InventorySlot,
    ) -> Result<()> {
        let slot_path = inventory_slot.get_serializations()[0];

        if let Some(slot) = self
            .character
            .equipped_inventory_list
            .iter_mut()
            .find(|s| s.slot_data_path == slot_path)
        {
            //Lock in character data
            slot.enabled = false;
        }

        if let Some(current_slot) = self
            .unlockable_inventory_slots
            .iter_mut()
            .find(|i| i.slot == *inventory_slot)
        {
            current_slot.unlocked = false;
        } else {
            self.unlockable_inventory_slots.push(InventorySlotData {
                slot: inventory_slot.to_owned(),
                unlocked: false,
            });
        }

        Ok(())
    }

    pub fn unlock_inventory_slot(&mut self, inventory_slot: &InventorySlot) -> Result<()> {
        let slot_path = inventory_slot.get_serializations()[0];

        let slot = self
            .character
            .equipped_inventory_list
            .iter_mut()
            .find(|s| s.slot_data_path == slot_path)
            .with_context(|| {
                format!(
                    "failed to find inventory slot: {}",
                    inventory_slot.to_string()
                )
            })?;

        //Unlock in character data
        slot.enabled = true;

        let class_mod_challenge = match self.player_class {
            PlayerClass::BeastMaster => Challenge::BeastMasterClassModSlot,
            PlayerClass::Gunner => Challenge::GunnerClassModSlot,
            PlayerClass::Operative => Challenge::OperativeClassModSlot,
            PlayerClass::Siren => Challenge::SirenClassModSlot,
        };

        let class_mod_challenge_path = class_mod_challenge.get_serializations()[0];

        match inventory_slot {
            InventorySlot::ClassMod => {
                self.unlock_challenge_obj(class_mod_challenge_path, 1, 0)?;
            }
            InventorySlot::Artifact => {
                self.unlock_challenge_obj(Challenge::ArtifactSlot.get_serializations()[0], 1, 0)?;
            }
            _ => (),
        }

        if let Some(current_slot) = self
            .unlockable_inventory_slots
            .iter_mut()
            .find(|i| i.slot == *inventory_slot)
        {
            current_slot.unlocked = true;
        } else {
            self.unlockable_inventory_slots.push(InventorySlotData {
                slot: inventory_slot.to_owned(),
                unlocked: true,
            });
        }

        Ok(())
    }

    pub fn sdu_slots(&self) -> &Vec<SaveSduSlotData> {
        &self.sdu_slots
    }

    pub fn set_sdu_slot(&mut self, sdu_slot: &SaveSduSlot, level: i32) {
        let sdu_path = sdu_slot.get_serializations()[0];

        if let Some(sdu) = self
            .character
            .sdu_list
            .iter_mut()
            .find(|s| s.sdu_data_path == sdu_path)
        {
            sdu.sdu_level = level;
        } else {
            self.character.sdu_list.push(OakSDUSaveGameData {
                sdu_level: level,
                sdu_data_path: sdu_path.to_string(),
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            })
        }

        if let Some(current_slot) = self.sdu_slots.iter_mut().find(|i| i.sdu == *sdu_slot) {
            current_slot.current = level;
        } else {
            self.sdu_slots.push(SaveSduSlotData {
                sdu: sdu_slot.to_owned(),
                current: level,
                max: sdu_slot.maximum(),
            });
        }
    }

    pub fn ammo_pools(&self) -> &Vec<AmmoPoolData> {
        &self.ammo_pools
    }

    pub fn set_ammo_pool(&mut self, ammo_pool: &AmmoPool, amount: i32) -> Result<()> {
        let pool_path = ammo_pool.get_serializations()[0];

        let pool = self
            .character
            .resource_pools
            .iter_mut()
            .find(|rp| rp.resource_path == pool_path)
            .with_context(|| format!("failed to find ammo pool: {}", ammo_pool.to_string()))?;

        pool.amount = amount as f32;

        if let Some(current_pool) = self.ammo_pools.iter_mut().find(|i| i.pool == *ammo_pool) {
            current_pool.current = amount;
        } else {
            self.ammo_pools.push(AmmoPoolData {
                pool: ammo_pool.to_owned(),
                current: amount,
                max: ammo_pool.maximum(),
            });
        }

        Ok(())
    }

    pub fn challenge_milestones(&self) -> &Vec<ChallengeData> {
        &self.challenge_milestones
    }

    pub fn vehicle_data(&self) -> &[VehicleData; 12] {
        &self.vehicle_data
    }

    pub fn unlock_vehicle_data(&mut self, vehicle_type: &VehicleType) {
        let data_set = vehicle_type.data_set();

        match vehicle_type.subtype() {
            VehicleSubType::Chassis => {
                for d in data_set {
                    if !self
                        .character
                        .vehicles_unlocked_data
                        .iter()
                        .any(|vd| vd.asset_path == d)
                    {
                        self.character
                            .vehicles_unlocked_data
                            .push(VehicleUnlockedSaveGameData {
                                asset_path: d.to_owned(),
                                just_unlocked: true,
                                unknown_fields: Default::default(),
                                cached_size: Default::default(),
                            });
                    }
                }
            }
            VehicleSubType::Skins | VehicleSubType::Parts => {
                for d in data_set {
                    if !self
                        .character
                        .vehicle_parts_unlocked
                        .contains(&d.to_owned())
                    {
                        self.character.vehicle_parts_unlocked.push(d.to_owned());
                    }
                }
            }
        }

        let existing_vd = self
            .vehicle_data
            .iter_mut()
            .find(|vd| vd.vehicle_type == *vehicle_type);

        if let Some(existing) = existing_vd {
            existing.current = vehicle_type.maximum();
        }
    }

    pub fn inventory_items(&self) -> &Vec<Bl3Item> {
        &self.inventory_items
    }

    pub fn inventory_items_mut(&mut self) -> &mut Vec<Bl3Item> {
        &mut self.inventory_items
    }

    pub fn create_inventory_item(
        pickup_order_index: i32,
        item: &Bl3Item,
        is_seen: bool,
    ) -> Result<OakInventoryItemSaveGameData> {
        let flags: i32 = if let Some(flags) = item.flags {
            flags.bits()
        } else {
            let mut default_flags = ItemFlags::empty();

            if is_seen {
                default_flags |= ItemFlags::SEEN;
            }

            default_flags.bits()
        };

        let item_serial_number = item.get_serial_number(true)?;

        let res = OakInventoryItemSaveGameData {
            item_serial_number,
            pickup_order_index,
            flags,
            weapon_skin_path: "".to_string(),
            development_save_data: Default::default(),
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        };

        Ok(res)
    }

    pub fn remove_inventory_item(&mut self, index: usize) {
        if index < self.character.inventory_items.len() {
            self.character.inventory_items.remove(index);
        }

        if index < self.inventory_items.len() {
            self.inventory_items.remove(index);
        }
    }

    pub fn add_inventory_item(&mut self, pickup_order_index: i32, item: &Bl3Item) -> Result<()> {
        let new_oak_item = Self::create_inventory_item(pickup_order_index, item, true)?;

        self.character.inventory_items.push(new_oak_item);

        self.inventory_items.push(item.to_owned());

        Ok(())
    }

    pub fn insert_inventory_item(
        &mut self,
        pickup_order_index: i32,
        item_index: usize,
        item: &Bl3Item,
    ) -> Result<()> {
        let new_oak_item = Self::create_inventory_item(pickup_order_index, item, true)?;

        self.character
            .inventory_items
            .insert(item_index, new_oak_item);

        self.inventory_items.insert(item_index, item.to_owned());

        Ok(())
    }

    pub fn replace_inventory_item(
        &mut self,
        pickup_order_index: i32,
        item_index: usize,
        new_item: &Bl3Item,
    ) -> Result<()> {
        self.insert_inventory_item(pickup_order_index, item_index, new_item)?;

        // Remove old item
        self.remove_inventory_item(item_index + 1);

        Ok(())
    }

    pub fn add_inventory_category_item(
        &mut self,
        base_category_definition_hash: u32,
        quantity: i32,
    ) {
        self.character
            .inventory_category_list
            .push(InventoryCategorySaveData {
                base_category_definition_hash,
                quantity,
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            });
    }

    pub fn unlock_challenge_obj(
        &mut self,
        challenge_obj: &str,
        completed_count: i32,
        progress_level: i32,
    ) -> Result<()> {
        let challenge = self
            .character
            .challenge_data
            .iter_mut()
            .find(|c| c.challenge_class_path == challenge_obj)
            .with_context(|| format!("failed to read challenge_obj: {}", challenge_obj))?;

        challenge.currently_completed = true;
        challenge.is_active = false;
        challenge.completed_count = completed_count;
        challenge.progress_counter = 0;
        challenge.completed_progress_level = progress_level;

        Ok(())
    }

    pub fn set_game_stat(&mut self, stat_path: &str, stat_value: i32) {
        if let Some(game_stat) = self
            .character
            .game_stats_data
            .iter_mut()
            .find(|s| s.stat_path == stat_path)
        {
            game_stat.stat_value = stat_value;
        } else {
            self.character.game_stats_data.push(GameStatSaveGameData {
                stat_path: stat_path.to_owned(),
                stat_value,
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            });
        }
    }
}
