#[derive(Debug)]
pub enum Currency {
    Money,
    Eridium,
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
