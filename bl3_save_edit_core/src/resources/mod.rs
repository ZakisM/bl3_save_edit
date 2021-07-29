use once_cell::sync::Lazy;

use crate::models::inventory_serial_db::InventorySerialDb;

pub const INVENTORY_SERIAL_DB_JSON: &[u8] =
    include_bytes!("../../resources/inventory_serial_db.json");

pub static INVENTORY_SERIAL_DB: Lazy<InventorySerialDb> =
    Lazy::new(|| InventorySerialDb::load().expect("failed to load inventory serial db"));

#[derive(Debug)]
pub struct InventoryPartBody {
    manufacturer: &'static str,
    rarity: &'static str,
    parts: Vec<InventoryPart>,
}

#[derive(Debug)]
pub struct InventoryPart {
    category: &'static str,
    parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Part {
    name: &'static str,
    min_parts: u8,
    max_parts: u8,
    dependencies: Option<Vec<&'static str>>,
    excluders: Option<Vec<&'static str>>,
}
