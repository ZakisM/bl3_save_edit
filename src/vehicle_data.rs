use strum::{Display, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VehicleStats {
    pub name: VehicleName,
    pub chassis_count: usize,
    pub total_chassis_count: usize,
    pub parts_count: usize,
    pub total_parts_count: usize,
    pub skins_count: usize,
    pub total_skins_count: usize,
}

#[derive(Debug, Eq, Display, EnumString, PartialEq, Ord, PartialOrd)]
pub enum VehicleName {
    Outrunner,
    Jetbeast,
    Technical,
    Cyclone,
}
