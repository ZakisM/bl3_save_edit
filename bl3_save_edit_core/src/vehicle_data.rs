use strum::Display;

use crate::game_data::{
    VEHICLE_CHASSIS_CYCLONE, VEHICLE_CHASSIS_JETBEAST, VEHICLE_CHASSIS_OUTRUNNER,
    VEHICLE_CHASSIS_TECHNICAL, VEHICLE_PARTS_CYCLONE, VEHICLE_PARTS_JETBEAST,
    VEHICLE_PARTS_OUTRUNNER, VEHICLE_PARTS_TECHNICAL, VEHICLE_SKINS_CYCLONE,
    VEHICLE_SKINS_JETBEAST, VEHICLE_SKINS_OUTRUNNER, VEHICLE_SKINS_TECHNICAL,
};

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct VehicleData {
    pub vehicle_type: VehicleType,
    pub current: usize,
}

impl VehicleData {
    pub fn new(vehicle_type: VehicleType, current: usize) -> Self {
        VehicleData {
            vehicle_type,
            current,
        }
    }
}

#[derive(Debug, Eq, Display, PartialEq, Ord, PartialOrd, Clone)]
pub enum VehicleType {
    Outrunner(VehicleSubType),
    Jetbeast(VehicleSubType),
    Technical(VehicleSubType),
    Cyclone(VehicleSubType),
}

#[derive(Debug, Eq, Display, PartialEq, Ord, PartialOrd, Clone)]
pub enum VehicleSubType {
    Chassis,
    Parts,
    Skins,
}

impl std::default::Default for VehicleType {
    fn default() -> Self {
        Self::Outrunner(VehicleSubType::Chassis)
    }
}

impl VehicleType {
    pub fn subtype(&self) -> &VehicleSubType {
        match self {
            VehicleType::Outrunner(sub_type) => sub_type,
            VehicleType::Jetbeast(sub_type) => sub_type,
            VehicleType::Technical(sub_type) => sub_type,
            VehicleType::Cyclone(sub_type) => sub_type,
        }
    }
    pub fn subtype_name(&self) -> String {
        self.subtype().to_string()
    }

    pub fn maximum(&self) -> usize {
        match self {
            VehicleType::Outrunner(sub_type) => match sub_type {
                VehicleSubType::Chassis => VEHICLE_CHASSIS_OUTRUNNER.len(),
                VehicleSubType::Parts => VEHICLE_PARTS_OUTRUNNER.len(),
                VehicleSubType::Skins => VEHICLE_SKINS_OUTRUNNER.len(),
            },
            VehicleType::Jetbeast(sub_type) => match sub_type {
                VehicleSubType::Chassis => VEHICLE_CHASSIS_JETBEAST.len(),
                VehicleSubType::Parts => VEHICLE_PARTS_JETBEAST.len(),
                VehicleSubType::Skins => VEHICLE_SKINS_JETBEAST.len(),
            },
            VehicleType::Technical(sub_type) => match sub_type {
                VehicleSubType::Chassis => VEHICLE_CHASSIS_TECHNICAL.len(),
                VehicleSubType::Parts => VEHICLE_PARTS_TECHNICAL.len(),
                VehicleSubType::Skins => VEHICLE_SKINS_TECHNICAL.len(),
            },
            VehicleType::Cyclone(sub_type) => match sub_type {
                VehicleSubType::Chassis => VEHICLE_CHASSIS_CYCLONE.len(),
                VehicleSubType::Parts => VEHICLE_PARTS_CYCLONE.len(),
                VehicleSubType::Skins => VEHICLE_SKINS_CYCLONE.len(),
            },
        }
    }

    pub fn data_set(&self) -> Vec<&str> {
        match self {
            VehicleType::Outrunner(sub_type) => match sub_type {
                VehicleSubType::Chassis => VEHICLE_CHASSIS_OUTRUNNER.to_vec(),
                VehicleSubType::Parts => VEHICLE_PARTS_OUTRUNNER.to_vec(),
                VehicleSubType::Skins => VEHICLE_SKINS_OUTRUNNER.to_vec(),
            },
            VehicleType::Jetbeast(sub_type) => match sub_type {
                VehicleSubType::Chassis => VEHICLE_CHASSIS_JETBEAST.to_vec(),
                VehicleSubType::Parts => VEHICLE_PARTS_JETBEAST.to_vec(),
                VehicleSubType::Skins => VEHICLE_SKINS_JETBEAST.to_vec(),
            },
            VehicleType::Technical(sub_type) => match sub_type {
                VehicleSubType::Chassis => VEHICLE_CHASSIS_TECHNICAL.to_vec(),
                VehicleSubType::Parts => VEHICLE_PARTS_TECHNICAL.to_vec(),
                VehicleSubType::Skins => VEHICLE_SKINS_TECHNICAL.to_vec(),
            },
            VehicleType::Cyclone(sub_type) => match sub_type {
                VehicleSubType::Chassis => VEHICLE_CHASSIS_CYCLONE.to_vec(),
                VehicleSubType::Parts => VEHICLE_PARTS_CYCLONE.to_vec(),
                VehicleSubType::Skins => VEHICLE_SKINS_CYCLONE.to_vec(),
            },
        }
    }
}
