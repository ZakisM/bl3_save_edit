use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::game_data::{
    VEHICLE_CHASSIS_CYCLONE, VEHICLE_CHASSIS_JETBEAST, VEHICLE_CHASSIS_OUTRUNNER, VEHICLE_CHASSIS_TECHNICAL, VEHICLE_PARTS_CYCLONE,
    VEHICLE_PARTS_JETBEAST, VEHICLE_PARTS_OUTRUNNER, VEHICLE_PARTS_TECHNICAL, VEHICLE_SKINS_CYCLONE, VEHICLE_SKINS_JETBEAST, VEHICLE_SKINS_OUTRUNNER,
    VEHICLE_SKINS_TECHNICAL,
};
use crate::protos::oak_profile::Profile;

#[derive(Debug)]
pub struct ProfileData {
    pub profile: Profile,
}

impl ProfileData {
    pub fn from_profile(profile: Profile) -> Result<Self> {
        dbg!(&profile);

        Ok(Self { profile })
    }
}
