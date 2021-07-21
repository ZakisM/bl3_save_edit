pub mod inventory_serial_db;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CustomFormatData {
    pub guid: Vec<u8>,
    pub entry: u32,
}
