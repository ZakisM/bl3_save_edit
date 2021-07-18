#[derive(Debug)]
pub struct FastTravelUnlockData<const T: usize> {
    objective_progress: [i32; T],
    mission_class_path: &'static str,
    active_objective_set_path: &'static str,
    active_travel_station_name: &'static str,
    discovered_level_name: &'static str,
    discovered_area_name: &'static str,
}

pub const TEST: FastTravelUnlockData<2> = FastTravelUnlockData {
    objective_progress: [10, 10],
    mission_class_path: "",
    active_objective_set_path: "",
    active_travel_station_name: "",
    discovered_level_name: "",
    discovered_area_name: "",
};
