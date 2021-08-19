use std::str::FromStr;

use anyhow::{Context, Result};
use derivative::Derivative;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use strum::{EnumMessage, IntoEnumIterator};

use crate::bl3_save::ammo::{AmmoPool, AmmoPoolData};
use crate::bl3_save::bl3_item::Bl3Item;
use crate::bl3_save::challenge_data::Challenge;
use crate::bl3_save::challenge_data::ChallengeData;
use crate::bl3_save::inventory_slot::{InventorySlot, InventorySlotData};
use crate::bl3_save::level_data::{LEVEL_CHALLENGES, LEVEL_STAT};
use crate::bl3_save::models::{Currency, VisitedTeleporter};
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
use crate::protos::oak_save::{Character, OakInventoryItemSaveGameData};
use crate::protos::oak_shared::{GameStatSaveGameData, OakSDUSaveGameData};
use crate::vehicle_data::{VehicleName, VehicleStats};

#[derive(Derivative)]
#[derivative(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct CharacterData {
    #[derivative(PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    pub character: Character,
    player_class: PlayerClass,
    player_level: i32,
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
    vehicle_stats: Vec<VehicleStats>,
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
        let guardian_rank = character
            .guardian_rank_character_data
            .as_ref()
            .map(|g| g.guardian_rank)
            .unwrap_or(0);

        let available_head_skins = PROFILE_HEADS_DEFAULTS
            .par_iter()
            .chain(PROFILE_HEADS.par_iter())
            .cloned()
            .filter(|h| {
                h.ident
                    .to_lowercase()
                    .contains(&player_class.to_string().to_lowercase())
            })
            .collect::<Vec<_>>();

        let head_skin_selected = available_head_skins
            .par_iter()
            .cloned()
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs == s.ident)
            })
            .unwrap_or(available_head_skins[0]);

        let available_character_skins = PROFILE_SKINS_DEFAULTS
            .par_iter()
            .chain(PROFILE_SKINS.par_iter())
            .cloned()
            .filter(|h| {
                h.ident
                    .to_lowercase()
                    .contains(&player_class.to_string().to_lowercase())
            })
            .collect::<Vec<_>>();

        let character_skin_selected = available_character_skins
            .par_iter()
            .cloned()
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs == s.ident)
            })
            .unwrap_or(available_character_skins[0]);

        let echo_theme_selected = PROFILE_ECHO_THEMES_DEFAULTS
            .par_iter()
            .chain(PROFILE_ECHO_THEMES.par_iter())
            .cloned()
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs == s.ident)
            })
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

        unlockable_inventory_slots.par_sort();

        let mut sdu_slots = character
            .sdu_list
            .par_iter()
            .map(|s| {
                let slot = SaveSduSlot::from_str(&s.sdu_data_path).with_context(|| {
                    format!("failed to read save sdu slot: {}", &s.sdu_data_path)
                })?;
                let additional_amount = slot.additional_amount();
                let max = slot.maximum();

                Ok(SaveSduSlotData {
                    slot,
                    current: s.sdu_level,
                    additional_amount,
                    max,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        // make sure that we include all sdu slots that might not be in our save
        SaveSduSlot::iter().for_each(|sdu| {
            let contains_sdu_slot = sdu_slots.par_iter().any(|save_sdu| {
                std::mem::discriminant(&sdu) == std::mem::discriminant(&save_sdu.slot)
            });

            let slot = sdu;
            let additional_amount = slot.additional_amount();
            let max = slot.maximum();

            if !contains_sdu_slot {
                sdu_slots.push(SaveSduSlotData {
                    slot,
                    current: 0,
                    additional_amount,
                    max,
                })
            }
        });

        sdu_slots.par_sort();

        let mut ammo_pools = character
            .resource_pools
            .par_iter()
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

        ammo_pools.par_sort();

        let mut challenge_milestones = Challenge::iter()
            .filter(|challenge| {
                let challenge_path = challenge.get_serializations()[0].to_lowercase();

                if challenge_path.contains("character") {
                    challenge_path.contains(&player_class.to_string().to_lowercase())
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

        challenge_milestones.par_sort();

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

        let vehicle_stats = vec![
            VehicleStats {
                name: VehicleName::Outrunner,
                chassis_count: outrunner_chassis,
                total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                parts_count: outrunner_parts,
                total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                skins_count: outrunner_skins,
                total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
            },
            VehicleStats {
                name: VehicleName::Jetbeast,
                chassis_count: jetbeast_chassis,
                total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                parts_count: jetbeast_parts,
                total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                skins_count: jetbeast_skins,
                total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
            },
            VehicleStats {
                name: VehicleName::Technical,
                chassis_count: technical_chassis,
                total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                parts_count: technical_parts,
                total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                skins_count: technical_skins,
                total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
            },
            VehicleStats {
                name: VehicleName::Cyclone,
                chassis_count: cyclone_chassis,
                total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                parts_count: cyclone_parts,
                total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                skins_count: cyclone_skins,
                total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
            },
        ];

        let inventory_items = character
            .inventory_items
            .par_iter()
            .filter_map(|i| Bl3Item::from_serial_bytes(i.item_serial_number.clone()).ok())
            .collect::<Vec<_>>();

        Ok(Self {
            character,
            player_class,
            player_level,
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
            vehicle_stats,
            inventory_items,
        })
    }

    pub fn player_class(&self) -> PlayerClass {
        self.player_class
    }

    pub fn set_player_class(&mut self, player_class: PlayerClass) -> Result<()> {
        self.player_class = player_class;

        let player_class_data = self
            .character
            .player_class_data
            .as_mut()
            .with_context(|| "failed to read Player Class data")?;

        player_class_data.player_class_path = player_class.get_serializations()[0].to_string();

        Ok(())
    }

    pub fn player_level(&self) -> i32 {
        self.player_level
    }

    pub fn set_player_level(&mut self, experience_points: i32) -> Result<()> {
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

        //Unlock skill tree
        if self.player_level > 1 && ability_data.tree_grade == 0 {
            ability_data.tree_grade = 2;
        }

        for (challenge_level, challenge_obj) in LEVEL_CHALLENGES {
            if self.player_level >= challenge_level {
                self.unlock_challenge_obj(challenge_obj, 1, 0)?;
            }
        }

        Ok(())
    }

    pub fn guardian_rank(&self) -> i32 {
        self.guardian_rank
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

    pub fn eridium(&self) -> i32 {
        self.eridium
    }

    pub fn playthroughs(&self) -> &Vec<Playthrough> {
        &self.playthroughs
    }

    pub fn unlockable_inventory_slots(&self) -> &Vec<InventorySlotData> {
        &self.unlockable_inventory_slots
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

    pub fn set_sdu_slots(&mut self, sdu_slot: &SaveSduSlot, level: i32) -> Result<()> {
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

        if let Some(current_slot) = self.sdu_slots.iter_mut().find(|i| i.slot == *sdu_slot) {
            current_slot.current = level;
        } else {
            self.sdu_slots.push(SaveSduSlotData {
                slot: sdu_slot.to_owned(),
                current: level,
                additional_amount: sdu_slot.additional_amount(),
                max: sdu_slot.maximum(),
            });
        }

        Ok(())
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

    pub fn vehicle_stats(&self) -> &Vec<VehicleStats> {
        &self.vehicle_stats
    }

    pub fn inventory_items(&self) -> &Vec<Bl3Item> {
        &self.inventory_items
    }

    pub fn create_inventory_item(
        item: &Bl3Item,
        pickup_order_index: i32,
        is_seen: bool,
        is_favourite: bool,
        is_trash: bool,
    ) -> Result<OakInventoryItemSaveGameData> {
        let mut flags = 0;

        if is_seen {
            flags |= 0x1;
        }

        if is_favourite {
            flags |= 0x2
        } else if is_trash {
            flags |= 0x4
        }

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

    pub fn add_inventory_item(&mut self, pickup_order_index: i32, item: &Bl3Item) -> Result<()> {
        let new_item = Self::create_inventory_item(item, pickup_order_index, true, true, false)?;

        self.character.inventory_items.push(new_item);

        self.inventory_items.push(item.to_owned());

        Ok(())
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

    pub fn set_active_travel_stations(
        &mut self,
        _playthrough_index: usize,
        _visited_teleporters_list: &[VisitedTeleporter],
    ) {
        //TODO: Find a save with every location and map everything below...

        // let mission_list = self.character.mission_playthroughs_data.get_mut(0);
        //
        // if let Some(mission_list) = mission_list {
        //     mission_list.mission_list.push(MissionStatusPlayerSaveGameData {
        //         status: MissionStatusPlayerSaveGameData_MissionState::MS_Complete,
        //         has_been_viewed_in_log: false,
        //         objectives_progress: vec![1, 1, 1, 0, 1, 1],
        //         mission_class_path: "/Game/Missions/Side/Slaughters/TechSlaughter/Mission_TechSlaughterDiscovery.Mission_TechSlaughterDiscovery_C".to_string(),
        //         active_objective_set_path: "/Game/Missions/Side/Slaughters/TechSlaughter/Mission_TechSlaughterDiscovery.Set_TalkToNPC_ObjectiveSet".to_string(),
        //         dlc_package_id: 0,
        //         kickoff_played: true,
        //         league_instance: 0,
        //         unknown_fields: Default::default(),
        //         cached_size: Default::default(),
        //     });
        //
        //     dbg!(&mission_list.mission_list);
        // }
        //
        // let curr_active_travel_stations = &mut self
        //     .character
        //     .active_travel_stations_for_playthrough
        //     .get_mut(playthrough_index)
        //     .expect("failed to read current active travel stations for playthrough: ")
        //     .active_travel_stations;
        //
        // curr_active_travel_stations.push(ActiveFastTravelSaveData {
        //     active_travel_station_name:
        //         "/Game/GameData/FastTravel/FTS_TechSlaughterDropPod.FTS_TechSlaughterDropPod"
        //             .to_owned(),
        //     blacklisted: false,
        //     unknown_fields: Default::default(),
        //     cached_size: Default::default(),
        // });
        //
        // let discovery_data = &mut self.character.discovery_data;
        //
        // if let Some(discovery_data) = discovery_data.as_mut() {
        //     discovery_data
        //         .discovered_level_info
        //         .push(DiscoveredLevelInfo {
        //             discovered_level_name: "/Game/Maps/Slaughters/TechSlaughter/TechSlaughter_P"
        //                 .to_string(),
        //             //the index of playthrough + 1 i think
        //             discovered_playthroughs: 1,
        //             discovered_area_info: RepeatedField::from_vec(vec![DiscoveredAreaInfo {
        //                 discovered_area_name: "TECHSLAUGHTER_PWDA_2".to_string(),
        //                 //the index of playthrough + 1 i think
        //                 discovered_playthroughs: 1,
        //                 unknown_fields: Default::default(),
        //                 cached_size: Default::default(),
        //             }]),
        //             unknown_fields: Default::default(),
        //             cached_size: Default::default(),
        //         });
        // }
        //
        // let challenge_data = &mut self.character.challenge_data;
        //
        // let challenge_we_want = challenge_data
        //     .iter_mut()
        //     .find(|cd| cd.challenge_class_path == "/Game/GameData/Challenges/Discovery/Slaughter_Tech/Challenge_Discovery_TechSlaughter1.Challenge_Discovery_TechSlaughter1_C");
        //
        // if let Some(challenge_we_want) = challenge_we_want {
        //     challenge_we_want.completed_count = 1;
        //     challenge_we_want.currently_completed = true;
        // }
        //
        // let challenge_we_want = challenge_data
        //     .iter_mut()
        //     .find(|cd| cd.challenge_class_path == "/Game/GameData/Challenges/FastTravel/Challenge_FastTravel_TechSlaughter1.Challenge_FastTravel_TechSlaughter1_C");
        //
        // if let Some(challenge_we_want) = challenge_we_want {
        //     challenge_we_want.completed_count = 1;
        //     challenge_we_want.currently_completed = true;
        // }

        let save_name = format!("{}-out.txt", self.character.save_game_id);
        let data = format!("{:#?}", self.character);

        std::fs::write(save_name, data).unwrap();

        // visited_teleporters_list.iter().for_each(|vt| {
        //     if vt.visited
        //         && !curr_active_travel_stations
        //             .iter()
        //             .any(|ats| ats.active_travel_station_name.to_lowercase() == vt.game_data.ident)
        //     {
        //         // println!("Adding: {}", vt.game_data.ident);
        //
        //         // curr_active_travel_stations.push(ActiveFastTravelSaveData {
        //         //     active_travel_station_name: vt.game_data.ident.to_owned(),
        //         //     blacklisted: false,
        //         //     unknown_fields: Default::default(),
        //         //     cached_size: Default::default(),
        //         // });
        //     } else if !vt.visited {
        //         if let Some(curr_station) = curr_active_travel_stations.iter().position(|ats| {
        //             ats.active_travel_station_name.to_lowercase() == vt.game_data.ident
        //         }) {
        //             // println!("Removing: {}", vt.game_data.ident);
        //
        //             // curr_active_travel_stations.remove(curr_station);
        //         }
        //     }
        // })
    }
}
