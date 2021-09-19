use iced::{Column, Container, Length};

use crate::bl3_ui::Bl3Message;
use crate::views::manage_save::vehicle::vehicle_unlocker::VehicleUnlocker;

pub mod vehicle_unlocker;

#[derive(Debug, Default)]
pub struct VehicleState {
    pub unlocker: VehicleUnlocker,
}

#[derive(Debug, Clone)]
pub enum SaveVehicleInteractionMessage {
    UnlockMessage(VehicleUnlockedMessage),
}

#[derive(Debug, Clone)]
pub enum VehicleUnlockedMessage {
    OutrunnerChassis(bool),
    OutrunnerParts(bool),
    OutrunnerSkins(bool),
    JetbeastChassis(bool),
    JetbeastParts(bool),
    JetbeastSkins(bool),
    TechnicalChassis(bool),
    TechnicalParts(bool),
    TechnicalSkins(bool),
    CycloneChassis(bool),
    CycloneParts(bool),
    CycloneSkins(bool),
}

pub fn view(vehicle_state: &mut VehicleState) -> Container<Bl3Message> {
    let vehicle_unlocker = vehicle_state.unlocker.view().width(Length::Fill);

    let all_contents = Column::new().push(vehicle_unlocker).spacing(20);

    Container::new(all_contents).padding(30)
}
