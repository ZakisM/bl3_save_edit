use std::convert::TryInto;

use crate::views::manage_save::fast_travel::PlaythroughType;
use crate::views::manage_save::ManageSaveState;

pub fn map_fast_travel_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    let last_play_through_index = save
        .character_data
        .character
        .last_play_through_index
        .try_into()
        .unwrap_or(0);

    manage_save_state
        .main_state
        .fast_travel_state
        .playthroughs_len = save.character_data.playthroughs.len();

    manage_save_state
        .main_state
        .fast_travel_state
        .playthrough_type_selected = match last_play_through_index {
        1 => PlaythroughType::Tvhm,
        _ => PlaythroughType::Normal,
    };

    manage_save_state
        .main_state
        .fast_travel_state
        .last_visited_teleporter_selected = save
        .character_data
        .playthroughs
        .get(last_play_through_index)
        .map(|p| p.current_map)
        .unwrap_or(
            manage_save_state
                .main_state
                .fast_travel_state
                .fast_travel_locations[0],
        );

    map_fast_travel_stations_to_visited_teleporters_list(
        last_play_through_index,
        manage_save_state,
    );
}

pub fn map_fast_travel_stations_to_visited_teleporters_list(
    playthrough_index: usize,
    manage_save_state: &mut ManageSaveState,
) {
    let save = &manage_save_state.current_file;

    if let Some(playthrough) = save.character_data.playthroughs.get(playthrough_index) {
        manage_save_state
            .main_state
            .fast_travel_state
            .visited_teleporters_list
            .iter_mut()
            .for_each(|vt| {
                vt.visited = false;

                if playthrough
                    .active_travel_stations
                    .iter()
                    .any(|ats| ats.to_lowercase() == vt.game_data.ident)
                {
                    vt.visited = true;
                }
            });
    }
}
