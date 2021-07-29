#[derive(Debug)]
pub struct FastTravelUnlockData<const T: usize, const U: usize> {
    pub objective_progress: [i32; T],
    pub mission_class_path: &'static str,
    pub active_objective_set_path: &'static str,
    pub active_travel_station_name: &'static str,
    pub discovered_level_name: &'static str,
    pub discovered_area_name: &'static str,
    pub challenges: [&'static str; U],
}

pub const AMBERMIRE: FastTravelUnlockData<0, 1> = FastTravelUnlockData {
    objective_progress: [],
    mission_class_path: "",
    active_objective_set_path: "/Game/GameData/Challenges/EchoLog_NonMission/Challenge_EchoLog_NonMission_Marshfields1.Challenge_EchoLog_NonMission_Marshfields1_C",
    active_travel_station_name: "/Game/GameData/FastTravel/FTS_Marshfields.FTS_Marshfields",
    discovered_level_name: "/Game/Maps/Zone_2/MarshFields/MarshFields_P",
    discovered_area_name: "MARSHFIELDS_COMBATWDA_1",
    challenges: [
        "/Game/GameData/Challenges/FastTravel/Challenge_FastTravel_Marshfields1.Challenge_FastTravel_Marshfields1_C"
    ],
};

//Have actually tested this and it works... more data needed for all travel stations i.e objective progress array
pub const SLAUGHTERSTAR_3000: FastTravelUnlockData<6, 2> = FastTravelUnlockData {
    objective_progress: [1, 1, 1, 0, 1, 1],
    mission_class_path: "/Game/Missions/Side/Slaughters/TechSlaughter/Mission_TechSlaughterDiscovery.Mission_TechSlaughterDiscovery_C",
    active_objective_set_path: "/Game/Missions/Side/Slaughters/TechSlaughter/Mission_TechSlaughterDiscovery.Set_TalkToNPC_ObjectiveSet",
    active_travel_station_name: "/Game/GameData/FastTravel/FTS_TechSlaughterDropPod.FTS_TechSlaughterDropPod",
    discovered_level_name: "/Game/Maps/Slaughters/TechSlaughter/TechSlaughter_P",
    discovered_area_name: "TECHSLAUGHTER_PWDA_2",
    challenges: [
        "/Game/GameData/Challenges/Discovery/Slaughter_Tech/Challenge_Discovery_TechSlaughter1.Challenge_Discovery_TechSlaughter1_C",
        "/Game/GameData/Challenges/FastTravel/Challenge_FastTravel_TechSlaughter1.Challenge_FastTravel_TechSlaughter1_C"
    ],
};
