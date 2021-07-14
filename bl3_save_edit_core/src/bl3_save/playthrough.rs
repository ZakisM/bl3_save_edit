use anyhow::{Context, Result};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;

use crate::bl3_save::util::{get_filtered_mission_list, IMPORTANT_MISSIONS};
use crate::game_data::{GameDataExt, FAST_TRAVEL, MISSION};
use crate::protos::oak_save::{Character, MissionStatusPlayerSaveGameData_MissionState};

#[derive(Debug, Clone)]
pub struct Playthrough {
    pub mayhem_level: i32,
    pub mayhem_random_seed: i32,
    pub current_map: String,
    pub active_missions: Vec<String>,
    pub missions_completed: Vec<String>,
    pub mission_milestones: Vec<String>,
    pub active_travel_stations: Vec<String>,
}

impl Playthrough {
    pub fn playthroughs_from_character(character: &Character) -> Result<Vec<Playthrough>> {
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
                    *MISSION,
                    mission_playthrough_data,
                    MissionStatusPlayerSaveGameData_MissionState::MS_Active,
                );

                let mut missions_completed = get_filtered_mission_list(
                    *MISSION,
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

                let active_travel_stations = character
                    .active_travel_stations_for_playthrough
                    .get(i)
                    .par_iter()
                    .map(|ats| {
                        ats.active_travel_stations
                            .par_iter()
                            .map(|ats| ats.active_travel_station_name.clone())
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect::<Vec<String>>();

                Ok(Playthrough {
                    mayhem_level,
                    mayhem_random_seed,
                    current_map,
                    active_missions,
                    missions_completed,
                    mission_milestones,
                    active_travel_stations,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(playthroughs)
    }
}
