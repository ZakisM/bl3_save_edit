use bl3_save_edit_core::resources::{
    INVENTORY_BALANCE_PARTS, INVENTORY_INV_DATA_PARTS, INVENTORY_MANUFACTURER_PARTS,
    INVENTORY_PARTS_ALL_CATEGORIZED, INVENTORY_SERIAL_DB, INVENTORY_SERIAL_DB_PARTS_CATEGORIZED,
};

use crate::config::Config;

pub async fn load_lazy_data() {
    println!("Loading lazy data...");

    let _ = &*INVENTORY_SERIAL_DB;
    let _ = &*INVENTORY_PARTS_ALL_CATEGORIZED;
    let _ = &*INVENTORY_SERIAL_DB_PARTS_CATEGORIZED;
    let _ = &*INVENTORY_BALANCE_PARTS;
    let _ = &*INVENTORY_INV_DATA_PARTS;
    let _ = &*INVENTORY_MANUFACTURER_PARTS;
}

pub async fn load_config() -> Config {
    println!("Loading config...");

    Config::load().await
}
