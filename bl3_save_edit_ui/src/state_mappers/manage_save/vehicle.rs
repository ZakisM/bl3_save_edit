use bl3_save_edit_core::bl3_save::Bl3Save;
use bl3_save_edit_core::vehicle_data::{VehicleSubType, VehicleType};

use crate::views::manage_save::vehicle::vehicle_unlocker::VehicleUnlocker;
use crate::views::manage_save::ManageSaveState;

pub fn map_save_to_vehicle_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    let mut unlocker = VehicleUnlocker::default();

    for vd in save.character_data.vehicle_data() {
        match &vd.vehicle_type {
            VehicleType::Outrunner(sub_type) => match sub_type {
                VehicleSubType::Chassis => {
                    unlocker.outrunner_chassis.vehicle_data.current = vd.current
                }
                VehicleSubType::Parts => unlocker.outrunner_parts.vehicle_data.current = vd.current,
                VehicleSubType::Skins => unlocker.outrunner_skins.vehicle_data.current = vd.current,
            },
            VehicleType::Jetbeast(sub_type) => match sub_type {
                VehicleSubType::Chassis => {
                    unlocker.jetbeast_chassis.vehicle_data.current = vd.current
                }
                VehicleSubType::Parts => unlocker.jetbeast_parts.vehicle_data.current = vd.current,
                VehicleSubType::Skins => unlocker.jetbeast_skins.vehicle_data.current = vd.current,
            },
            VehicleType::Technical(sub_type) => match sub_type {
                VehicleSubType::Chassis => {
                    unlocker.technical_chassis.vehicle_data.current = vd.current
                }
                VehicleSubType::Parts => unlocker.technical_parts.vehicle_data.current = vd.current,
                VehicleSubType::Skins => unlocker.technical_skins.vehicle_data.current = vd.current,
            },
            VehicleType::Cyclone(sub_type) => match sub_type {
                VehicleSubType::Chassis => {
                    unlocker.cyclone_chassis.vehicle_data.current = vd.current
                }
                VehicleSubType::Parts => unlocker.cyclone_parts.vehicle_data.current = vd.current,
                VehicleSubType::Skins => unlocker.cyclone_skins.vehicle_data.current = vd.current,
            },
        }
    }

    manage_save_state.save_view_state.vehicle_state.unlocker = unlocker;
}

pub fn map_vehicle_state_to_save(manage_save_state: &mut ManageSaveState, save: &mut Bl3Save) {
    let vehicle_state = &manage_save_state.save_view_state.vehicle_state;

    let unlocker = &vehicle_state.unlocker;

    let all_vehicle_unlock_boxes = [
        &unlocker.outrunner_chassis,
        &unlocker.outrunner_parts,
        &unlocker.outrunner_skins,
        &unlocker.jetbeast_chassis,
        &unlocker.jetbeast_parts,
        &unlocker.jetbeast_skins,
        &unlocker.technical_chassis,
        &unlocker.technical_parts,
        &unlocker.technical_skins,
        &unlocker.cyclone_chassis,
        &unlocker.cyclone_parts,
        &unlocker.cyclone_skins,
    ];

    for vd in all_vehicle_unlock_boxes {
        if vd.is_unlocked {
            save.character_data
                .unlock_vehicle_data(&vd.vehicle_data.vehicle_type)
        }
    }
}
