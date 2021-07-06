use anyhow::{Context, Result};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;

use crate::bl3_save::models::{Currency, Playthrough};
use crate::game_data::FAST_TRAVEL;
use crate::game_data::MISSION;
use crate::game_data::{GameDataExt, GameDataKv};
use crate::protos::oak_save::{
    Character, MissionPlaythroughSaveGameData, MissionStatusPlayerSaveGameData_MissionState,
};

pub const REQUIRED_XP_LIST: [[i32; 2]; 80] = [
    [0, 1],
    [358, 2],
    [1241, 3],
    [2850, 4],
    [5376, 5],
    [8997, 6],
    [13886, 7],
    [20208, 8],
    [28126, 9],
    [37798, 10],
    [49377, 11],
    [63016, 12],
    [78861, 13],
    [97061, 14],
    [117757, 15],
    [141092, 16],
    [167206, 17],
    [196238, 18],
    [228322, 19],
    [263595, 20],
    [302190, 21],
    [344238, 22],
    [389873, 23],
    [439222, 24],
    [492414, 25],
    [549578, 26],
    [610840, 27],
    [676325, 28],
    [746158, 29],
    [820463, 30],
    [899363, 31],
    [982980, 32],
    [1071435, 33],
    [1164850, 34],
    [1263343, 35],
    [1367034, 36],
    [1476041, 37],
    [1590483, 38],
    [1710476, 39],
    [1836137, 40],
    [1967582, 41],
    [2104926, 42],
    [2248285, 43],
    [2397772, 44],
    [2553501, 45],
    [2715586, 46],
    [2884139, 47],
    [3059273, 48],
    [3241098, 49],
    [3429728, 50],
    [3625271, 51],
    [3827840, 52],
    [4037543, 53],
    [4254491, 54],
    [4478792, 55],
    [4710556, 56],
    [4949890, 57],
    [5196902, 58],
    [5451701, 59],
    [5714393, 60],
    [5985086, 61],
    [6263885, 62],
    [6550897, 63],
    [6846227, 64],
    [7149982, 65],
    [7462266, 66],
    [7783184, 67],
    [8112840, 68],
    [8451340, 69],
    [8798786, 70],
    [9155282, 71],
    [9520932, 72],
    [9895837, 73],
    [10280103, 74],
    [10673830, 75],
    [11077120, 76],
    [11490077, 77],
    [11912801, 78],
    [12345393, 79],
    [12787955, 80],
];

pub const IMPORTANT_MISSIONS: [[&str; 2]; 7] = [
    ["Divine Retribution", "Main Game"],
    [
        "All Bets Off",
        "DLC1 - Moxxi's Heist of the Handsome Jackpot",
    ],
    ["The Call of Gythian", "DLC2 - Guns, Love, and Tentacles"],
    ["Riding to Ruin", "DLC3 - Bounty of Blood"],
    [
        "Locus of Rage",
        "DLC4 - Psycho Krieg and the Fantastic Fustercluck",
    ],
    ["Arms Race", "DLC5 - Designer's Cut"],
    [
        "Mysteriouslier: Horror at Scryer's Crypt",
        "DLC6 - Director's Cut",
    ],
];

pub const IMPORTANT_CHALLENGES: [[&str; 2]; 8] = [
    [
        "/Game/GameData/Challenges/Account/Challenge_VaultReward_Analyzer.Challenge_VaultReward_Analyzer_C",
        "Eridian Analyzer",
    ],
    [
        "/Game/GameData/Challenges/Account/Challenge_VaultReward_Resonator.Challenge_VaultReward_Resonator_C",
        "Eridian Resonator",
    ],
    [
        "/Game/GameData/Challenges/Account/Challenge_VaultReward_Mayhem.Challenge_VaultReward_Mayhem_C",
        "Mayhem Mode",
    ],
    [
        "/Game/GameData/Challenges/Account/Challenge_VaultReward_ArtifactSlot.Challenge_VaultReward_ArtifactSlot_C",
        "Artifact Slot",
    ],
    [
        "/Game/GameData/Challenges/Character/Beastmaster/BP_Challenge_Beastmaster_ClassMod.BP_Challenge_Beastmaster_ClassMod_C",
        "BeastMaster Class Mod Slot",
    ],
    [
        "/Game/GameData/Challenges/Character/Gunner/BP_Challenge_Gunner_ClassMod.BP_Challenge_Gunner_ClassMod_C",
        "Gunner Class Mod Slot",
    ],
    [
        "/Game/GameData/Challenges/Character/Operative/BP_Challenge_Operative_ClassMod.BP_Challenge_Operative_ClassMod_C",
        "Operative Class Mod Slot",
    ],
    [
        "/Game/GameData/Challenges/Character/Siren/BP_Challenge_Siren_ClassMod.BP_Challenge_Siren_ClassMod_C",
        "Siren Class Mod Slot",
    ],
];

pub fn currency_amount_from_character(character: &Character, currency: &Currency) -> i32 {
    character
        .inventory_category_list
        .par_iter()
        .find_first(|i| {
            i.base_category_definition_hash
                == match currency {
                    Currency::Money => 618814354,
                    Currency::Eridium => 3679636065,
                }
        })
        .map(|i| i.quantity)
        .unwrap_or(0)
}

pub fn experience_to_level(experience: i32) -> Result<i32> {
    REQUIRED_XP_LIST
        .iter()
        .rev()
        .find(|[xp, _]| experience >= *xp)
        .map(|[_, level]| *level)
        .with_context(|| {
            format!(
                "could not calculate level based off of experience: {}",
                experience
            )
        })
}

pub fn read_playthroughs(character: &Character) -> Result<Vec<Playthrough>> {
    let playthroughs = character
        .game_state_save_data_for_playthrough
        .par_iter()
        .enumerate()
        .map(|(i, playthrough)| {
            let mayhem_level = playthrough.mayhem_level;
            let mayhem_random_seed = playthrough.mayhem_random_seed;
            let current_map = character
                .last_active_travel_station_for_playthrough
                .get(i)
                .and_then(|m| FAST_TRAVEL.get_value_by_key(&m.to_lowercase()).ok())
                .map(|m| m.to_string())
                .context("failed to read character current map")?;

            let mission_playthrough_data = character
                .mission_playthroughs_data
                .get(i)
                .context("failed to read character active missions")?;

            let mut active_missions = get_filtered_mission_list(
                MISSION,
                mission_playthrough_data,
                MissionStatusPlayerSaveGameData_MissionState::MS_Active,
            );

            let mut missions_completed = get_filtered_mission_list(
                MISSION,
                mission_playthrough_data,
                MissionStatusPlayerSaveGameData_MissionState::MS_Complete,
            );

            active_missions.par_sort_unstable();
            missions_completed.par_sort_unstable();

            let mission_milestones = IMPORTANT_MISSIONS
                .par_iter()
                .filter(|[k, _]| missions_completed.par_iter().any(|m| *k == m))
                .map(|[_k, v]| v.to_string())
                .collect::<Vec<_>>();

            Ok(Playthrough {
                mayhem_level,
                mayhem_random_seed,
                current_map,
                active_missions,
                missions_completed,
                mission_milestones,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(playthroughs)
}

fn get_filtered_mission_list<const LENGTH: usize>(
    all_missions: [GameDataKv; LENGTH],
    m: &MissionPlaythroughSaveGameData,
    status: MissionStatusPlayerSaveGameData_MissionState,
) -> Vec<String> {
    m.mission_list
        .par_iter()
        .filter(|ms| ms.status == status)
        .map(|ms| {
            all_missions
                .iter()
                .find(|gd| ms.mission_class_path.to_lowercase().contains(gd.0 .0))
                .map(|gd| gd.0 .1.to_string())
                .unwrap_or_else(|| ms.mission_class_path.to_owned())
        })
        .collect()
}
