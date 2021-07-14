use crate::game_data::GameDataKv;

#[derive(Debug, Clone)]
pub enum Currency {
    Money,
    Eridium,
}

#[derive(Debug, Default)]
pub struct VisitedTeleporter {
    pub game_data: GameDataKv,
    pub visited: bool,
}
