use once_cell::sync::Lazy;

use crate::models::inventory_serial_db::InventorySerialDb;

pub const INVENTORY_SERIAL_DB_JSON: &[u8] =
    include_bytes!("../../resources/inventory_serial_db.json");

pub static INVENTORY_SERIAL_DB: Lazy<InventorySerialDb> =
    Lazy::new(|| InventorySerialDb::load().expect("failed to load inventory serial db"));
