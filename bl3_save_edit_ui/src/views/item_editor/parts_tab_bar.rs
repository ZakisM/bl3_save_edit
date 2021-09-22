use strum::Display;

#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum AvailablePartType {
    #[strum(to_string = "Available Parts")]
    Parts,
    #[strum(to_string = "Available Anointments")]
    Anointments,
}

impl std::default::Default for AvailablePartType {
    fn default() -> Self {
        Self::Parts
    }
}

#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum CurrentPartType {
    #[strum(to_string = "Current Parts")]
    Parts,
    #[strum(to_string = "Current Anointments")]
    Anointments,
}

impl std::default::Default for CurrentPartType {
    fn default() -> Self {
        Self::Parts
    }
}
