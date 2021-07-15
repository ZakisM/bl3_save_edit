use std::convert::TryInto;

use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::manage_save::fast_travel::PlaythroughType;
use crate::views::manage_save::ManageSaveState;

pub fn map_fast_travel_state(save: &Bl3Save, manage_save_state: &mut ManageSaveState) {
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

    if let Some(playthrough) = save
        .character_data
        .playthroughs
        .get(last_play_through_index)
    {
        manage_save_state
            .main_state
            .fast_travel_state
            .visited_teleporters_list
            .iter_mut()
            .for_each(|vt| {
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
