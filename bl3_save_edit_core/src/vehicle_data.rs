use strum::Display;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct VehicleStats {
    pub name: VehicleName,
    pub chassis_count: usize,
    pub total_chassis_count: usize,
    pub parts_count: usize,
    pub total_parts_count: usize,
    pub skins_count: usize,
    pub total_skins_count: usize,
}

#[derive(Debug, Eq, Display, PartialEq, Ord, PartialOrd, Clone)]
pub enum VehicleName {
    Outrunner,
    Jetbeast,
    Technical,
    Cyclone,
}
