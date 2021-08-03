use std::collections::{BTreeMap, BTreeSet, HashMap};

use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::Deserialize;

use crate::models::inventory_serial_db::InventorySerialDb;

pub const INVENTORY_SERIAL_DB_JSON: &str = include_str!("../../resources/inventory_serial_db.json");

const INVENTORY_PARTS_SHIELDS_DATA: &[u8] =
    include_bytes!("../../resources/INVENTORY_PARTS_SHIELDS.csv");

pub static INVENTORY_SERIAL_DB: Lazy<InventorySerialDb> =
    Lazy::new(|| InventorySerialDb::load().expect("failed to load inventory serial db"));

#[derive(Debug, Deserialize)]
struct InventoryItemRecord {
    #[serde(rename = "Manufacturer/Name")]
    manufacturer: String,
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

#[derive(Debug)]
pub struct InventoryItem {
    pub manufacturer: String,
    pub rarity: String,
    pub inventory_categorized_parts: Vec<InventoryCategorizedParts>,
}

#[derive(Debug)]
pub struct InventoryCategorizedParts {
    pub category: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Part {
    pub name: String,
    pub min_parts: u8,
    pub max_parts: u8,
    pub dependencies: Option<Vec<String>>,
    pub excluders: Option<Vec<String>>,
}

pub static INVENTORY_PARTS_SHIELDS: Lazy<HashMap<String, InventoryItem>> = Lazy::new(|| {
    let parts_grouped = load_inventory_parts_grouped(INVENTORY_PARTS_SHIELDS_DATA);

    let mut m = HashMap::new();

    for (header, body) in parts_grouped {
        let inventory_categorized_parts = body
            .into_par_iter()
            .map(|(category, parts)| InventoryCategorizedParts {
                category,
                parts: parts
                    .par_iter()
                    .map(|p| Part {
                        name: p.name.to_owned(),
                        min_parts: p.min_parts,
                        max_parts: p.max_parts,
                        dependencies: p.dependencies.clone(),
                        excluders: p.excluders.clone(),
                    })
                    .collect::<Vec<_>>(),
            })
            .collect::<Vec<_>>();

        let inv_part = InventoryItem {
            manufacturer: header.manufacturer,
            rarity: header.rarity,
            inventory_categorized_parts,
        };

        m.insert(header.balance, inv_part);
    }

    m
});

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct TempHeader {
    manufacturer: String,
    rarity: String,
    balance: String,
}

type TempBody = BTreeMap<String, BTreeSet<Part>>;

fn load_inventory_parts_grouped(bytes: &[u8]) -> BTreeMap<TempHeader, TempBody> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bytes);

    let records = rdr.deserialize().map(|r| {
        let mut record: InventoryItemRecord =
            r.expect("failed to deserialize inventory part record");

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
    });

    let mut parts_grouped = BTreeMap::new();

    records.into_iter().for_each(|inv_part| {
        let inventory_part_header = TempHeader {
            manufacturer: inv_part.manufacturer,
            rarity: inv_part.rarity,
            balance: inv_part.balance,
        };

        let inventory_part = Part {
            name: inv_part.part,
            min_parts: inv_part.min_parts,
            max_parts: inv_part.max_parts,
            dependencies: inv_part.dependencies,
            excluders: inv_part.excluders,
        };

        let curr_group = parts_grouped
            .entry(inventory_part_header)
            .or_insert_with(BTreeMap::new);

        let curr_group_category = curr_group
            .entry(inv_part.category)
            .or_insert_with(BTreeSet::new);

        curr_group_category.insert(inventory_part);
    });

    parts_grouped
}