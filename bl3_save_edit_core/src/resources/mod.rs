use std::collections::HashMap;

use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use tracing::info;

use crate::bl3_item::{BalancePart, Bl3Item, InvDataPart, ManufacturerPart};
use crate::models::inventory_serial_db::InventorySerialDb;

type InventoryPartsAll = HashMap<String, ResourceItem>;
type InventorySerialDbCategorizedParts = HashMap<String, Vec<ResourceCategorizedParts>>;

pub const INVENTORY_SERIAL_DB_JSON_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_SERIAL_DB.json.sz");

const INVENTORY_PARTS_ALL_CATEGORIZED_RON_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_PARTS_ALL_CATEGORIZED.ron.sz");

const INVENTORY_SERIAL_DB_PARTS_CATEGORIZED_RON_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_SERIAL_DB_PARTS_CATEGORIZED.ron.sz");

const INVENTORY_BALANCE_PARTS_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_BALANCE_PARTS.ron.sz");

const INVENTORY_INV_DATA_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_INV_DATA_PARTS.ron.sz");

const INVENTORY_MANUFACTURER_PARTS_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_MANUFACTURER_PARTS.ron.sz");

const LOOTLEMON_ITEMS_COMPRESSED: &[u8] = include_bytes!("../../resources/LOOTLEMON_ITEMS.ron.sz");

pub static INVENTORY_SERIAL_DB: Lazy<InventorySerialDb> =
    Lazy::new(|| InventorySerialDb::load().expect("failed to load inventory serial db"));

pub static INVENTORY_PARTS_ALL_CATEGORIZED: Lazy<InventoryPartsAll> =
    Lazy::new(|| load_compressed_data(INVENTORY_PARTS_ALL_CATEGORIZED_RON_COMPRESSED));

pub static INVENTORY_SERIAL_DB_PARTS_CATEGORIZED: Lazy<InventorySerialDbCategorizedParts> =
    Lazy::new(|| load_compressed_data(INVENTORY_SERIAL_DB_PARTS_CATEGORIZED_RON_COMPRESSED));

pub static INVENTORY_BALANCE_PARTS: Lazy<Vec<BalancePart>> =
    Lazy::new(|| load_compressed_data(INVENTORY_BALANCE_PARTS_COMPRESSED));

pub static INVENTORY_INV_DATA_PARTS: Lazy<Vec<InvDataPart>> =
    Lazy::new(|| load_compressed_data(INVENTORY_INV_DATA_COMPRESSED));

pub static INVENTORY_MANUFACTURER_PARTS: Lazy<Vec<ManufacturerPart>> =
    Lazy::new(|| load_compressed_data(INVENTORY_MANUFACTURER_PARTS_COMPRESSED));

pub static LOOTLEMON_ITEMS: Lazy<Vec<LootlemonItem>> = Lazy::new(|| {
    let items = load_compressed_data::<Vec<LootlemonItemRaw>>(LOOTLEMON_ITEMS_COMPRESSED);

    let start_time = std::time::Instant::now();

    let mut lootlemon_items = items
        .into_par_iter()
        .map(|i| LootlemonItem {
            item: Bl3Item::from_serial_base64(&i.serial).expect("Failed to read Lootlemon Item"),
            link: i.link,
        })
        .collect::<Vec<_>>();

    lootlemon_items.par_sort_by_key(|i| i.item.balance_part().name.to_owned());

    if let Some(end_time) = std::time::Instant::now().checked_duration_since(start_time) {
        info!(
            "Read {} Lootlemon items in {} milliseconds",
            lootlemon_items.len(),
            end_time.as_millis()
        );
    }

    lootlemon_items
});

pub fn load_compressed_data<T: DeserializeOwned>(input: &'static [u8]) -> T {
    let mut rdr = snap::read::FrameDecoder::new(input);

    ron::de::from_reader(&mut rdr).expect("failed to read compressed data")
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ResourceItem {
    pub manufacturer: String,
    pub rarity: String,
    pub inventory_categorized_parts: Vec<ResourceCategorizedParts>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ResourceCategorizedParts {
    pub category: String,
    pub parts: Vec<ResourcePart>,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize)]
pub struct ResourcePart {
    pub name: String,
    pub min_parts: u8,
    pub max_parts: u8,
    pub dependencies: Option<Vec<String>>,
    pub excluders: Option<Vec<String>>,
    pub info: ResourcePartInfo,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize)]
pub struct ResourcePartInfo {
    pub positives: Option<String>,
    pub negatives: Option<String>,
    pub effects: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LootlemonItemRaw {
    pub serial: String,
    pub link: String,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LootlemonItem {
    pub item: Bl3Item,
    pub link: String,
}
