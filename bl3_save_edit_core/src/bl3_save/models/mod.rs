#[derive(Debug, Clone)]
pub enum Currency {
    Money,
    Eridium,
}

impl Currency {
    pub fn hash_value(&self) -> u32 {
        match self {
            Currency::Money => 618814354,
            Currency::Eridium => 3679636065,
        }
    }
}
