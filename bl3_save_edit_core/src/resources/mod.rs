use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::Read;

use heck::TitleCase;
use once_cell::sync::Lazy;
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use serde::Deserialize;

use crate::models::inventory_serial_db::InventorySerialDb;

type InventoryPartsAll = HashMap<String, ResourceItem>;
type InventorySerialDbCategorizedParts = HashMap<String, Vec<ResourceCategorizedParts>>;

pub const INVENTORY_SERIAL_DB_JSON_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_SERIAL_DB.json.sz");

const INVENTORY_PARTS_ALL_DATA_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_PARTS_ALL.csv.sz");

pub static INVENTORY_SERIAL_DB: Lazy<InventorySerialDb> =
    Lazy::new(|| InventorySerialDb::load().expect("failed to load inventory serial db"));

pub static INVENTORY_PARTS: Lazy<InventoryParts> =
    Lazy::new(load_inventory_parts_and_inventory_serial_parts);

pub struct InventoryParts {
    pub inventory_parts_all: InventoryPartsAll,
    // pub inventory_serial_db_categorized_parts: InventorySerialDbCategorizedParts,
}

#[derive(Debug, Clone, Deserialize)]
struct ResourceItemRecord {
    #[serde(rename = "Name")]
    manufacturer: String,
    #[serde(rename = "Weapon Type", skip)]
    weapon_type: Option<String>,
    #[serde(rename = "Rarity")]
    rarity: String,
    #[serde(rename = "Balance")]
    balance: String,
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Min Parts")]
    min_parts: u8,
    #[serde(rename = "Max Parts")]
    max_parts: u8,
    #[serde(rename = "Weight")]
    weight: f32,
    #[serde(rename = "Part")]
    part: String,
    #[serde(rename = "Dependencies")]
    dependencies: Option<Vec<String>>,
    #[serde(rename = "Excluders")]
    excluders: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone)]
pub struct ResourceItem {
    pub manufacturer: String,
    pub rarity: String,
    pub inventory_categorized_parts: Vec<ResourceCategorizedParts>,
}

#[derive(Debug, Default, Clone)]
pub struct ResourceCategorizedParts {
    pub category: String,
    pub parts: Vec<ResourcePart>,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ResourcePart {
    pub name: String,
    pub min_parts: u8,
    pub max_parts: u8,
    pub dependencies: Option<Vec<String>>,
    pub excluders: Option<Vec<String>>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct TempHeader {
    manufacturer: String,
    rarity: String,
    balance: String,
}

fn load_inventory_parts_and_inventory_serial_parts() -> InventoryParts {
    let records = load_inventory_parts_all_records();

    let inventory_serial_db_categorized_parts =
        load_inventory_serial_db_categorized_parts(&records);
    let inventory_parts_all = load_inventory_parts_all(records);

    InventoryParts {
        inventory_parts_all,
        // inventory_serial_db_categorized_parts,
    }
}

fn load_inventory_parts_all_records() -> Vec<ResourceItemRecord> {
    let bytes = &*INVENTORY_PARTS_ALL_DATA_COMPRESSED;

    let mut rdr = snap::read::FrameDecoder::new(bytes);

    let mut decompressed_bytes = String::new();

    rdr.read_to_string(&mut decompressed_bytes)
        .expect("Failed to read decompressed bytes");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(decompressed_bytes.as_bytes());

    let inventory_serial_db_all_parts = INVENTORY_SERIAL_DB.par_all_parts();

    rdr.deserialize()
        .par_bridge()
        .map(|r| {
            let record: ResourceItemRecord = r.expect("failed to deserialize resource part record");
            record
        })
        .filter(|r| inventory_serial_db_all_parts.contains(r.part.as_str()))
        .map(|mut record| {
            if let Some(curr_dependencies) = &record.dependencies {
                let all_dependencies = curr_dependencies
                    .get(0)
                    .expect("failed to read curr_dependency")
                    .split(',')
                    .map(|s| s.trim().to_owned())
                    .collect::<Vec<_>>();

                record.dependencies = Some(all_dependencies);
            };

            if let Some(curr_excluders) = &record.excluders {
                let all_excluders = curr_excluders
                    .get(0)
                    .expect("failed to read curr_excluder")
                    .split(',')
                    .map(|s| s.trim().to_owned())
                    .collect::<Vec<_>>();

                record.excluders = Some(all_excluders);
            };

            record
        })
        .collect::<Vec<_>>()
}

// TODO: Handle any errors here properly and exit UI correctly
fn load_inventory_parts_all(records: Vec<ResourceItemRecord>) -> HashMap<String, ResourceItem> {
    let parts_grouped = records
        .into_iter()
        .fold(BTreeMap::new(), |mut curr, inv_part| {
            let inventory_part_header = TempHeader {
                manufacturer: inv_part.manufacturer,
                rarity: inv_part.rarity,
                balance: inv_part.balance,
            };

            let inventory_part = ResourcePart {
                name: inv_part.part,
                min_parts: inv_part.min_parts,
                max_parts: inv_part.max_parts,
                dependencies: inv_part.dependencies,
                excluders: inv_part.excluders,
            };

            let curr_group = curr
                .entry(inventory_part_header)
                .or_insert_with(BTreeMap::new);

            let curr_group_category = curr_group
                .entry(inv_part.category.to_title_case())
                .or_insert_with(BTreeSet::new);

            curr_group_category.insert(inventory_part);

            curr
        });

    parts_grouped
        .into_iter()
        .fold(HashMap::new(), |mut curr, (header, body)| {
            let inventory_categorized_parts = body
                .into_par_iter()
                .map(|(category, parts)| ResourceCategorizedParts {
                    category,
                    parts: parts
                        .par_iter()
                        .map(|p| ResourcePart {
                            name: p.name.to_owned(),
                            min_parts: p.min_parts,
                            max_parts: p.max_parts,
                            dependencies: p.dependencies.clone(),
                            excluders: p.excluders.clone(),
                        })
                        .collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>();

            let inv_part = ResourceItem {
                manufacturer: header.manufacturer,
                rarity: header.rarity,
                inventory_categorized_parts,
            };

            curr.insert(header.balance, inv_part);

            curr
        })
}

// This will first get all of the parts found in INVENTORY_SERIAL_DB,
// then get part info from INVENTORY_PARTS_ALL.csv and group accordingly.
fn load_inventory_serial_db_categorized_parts(
    records: &[ResourceItemRecord],
) -> HashMap<String, Vec<ResourceCategorizedParts>> {
    let inventory_serial_db = &*INVENTORY_SERIAL_DB;
    let records = records;

    inventory_serial_db
        .data
        .entries()
        .par_bridge()
        .map(|(inv_db_category, _)| {
            let parts_grouped = inventory_serial_db.data[inv_db_category]["assets"]
                .members()
                .filter_map(|p| p.to_string().rsplit('.').next().map(|s| s.to_owned()))
                .fold(BTreeMap::new(), |mut curr, inv_db_part_name| {
                    let curr_record = records
                        .par_iter()
                        .find_first(|r| r.part == inv_db_part_name)
                        .map(|r| r.to_owned());

                    if let Some(curr_record) = curr_record {
                        let curr_group = curr
                            .entry(curr_record.category.to_title_case())
                            .or_insert_with(BTreeSet::new);

                        let inventory_part = ResourcePart {
                            name: curr_record.part,
                            min_parts: curr_record.min_parts,
                            max_parts: curr_record.max_parts,
                            dependencies: curr_record.dependencies,
                            excluders: curr_record.excluders,
                        };

                        curr_group.insert(inventory_part);
                    } else {
                        let curr_group = curr
                            .entry("Unknown Parts".to_owned())
                            .or_insert_with(BTreeSet::new);

                        let inventory_part = ResourcePart {
                            name: inv_db_part_name,
                            min_parts: 0,
                            max_parts: 0,
                            dependencies: None,
                            excluders: None,
                        };

                        curr_group.insert(inventory_part);
                    }

                    curr
                })
                .into_par_iter()
                .map(|(category, parts)| ResourceCategorizedParts {
                    category,
                    parts: parts.into_par_iter().collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>();

            (inv_db_category.to_owned(), parts_grouped)
        })
        .collect()
}
