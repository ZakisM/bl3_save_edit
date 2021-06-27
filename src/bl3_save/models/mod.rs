#[derive(Debug)]
pub enum Currency {
    Money,
    Eridium,
}

#[derive(Debug)]
pub struct ChallengeData {
    pub challenge: String,
    pub unlocked: bool,
}

#[derive(Debug)]
pub struct Playthrough {
    pub mayhem_level: i32,
    pub mayhem_random_seed: i32,
    pub current_map: String,
    pub active_missions: Vec<String>,
    pub missions_completed: Vec<String>,
    pub mission_milestones: Vec<String>,
}

#[derive(Debug)]
pub struct VehicleStats {
    pub name: String,
    pub chassis_count: usize,
    pub total_chassis_count: usize,
    pub parts_count: usize,
    pub total_parts_count: usize,
    pub skins_count: usize,
    pub total_skins_count: usize,
}
