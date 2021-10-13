use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write as Write2;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use csv::StringRecord;
use heck::TitleCase;
use json::JsonValue;
use protobuf_codegen_pure::{Codegen, Customize};
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use rayon::prelude::ParallelSliceMut;
use serde::{Deserialize, Serialize};

fn main() {
    let proto_inputs = [
        "protobufs/oak_profile.proto",
        "protobufs/oak_save.proto",
        "protobufs/oak_shared.proto",
    ];

    let game_data_inputs_kv = vec![
        "game_data/FAST_TRAVEL.csv",
        "game_data/MISSION.csv",
        "game_data/PROFILE_ROOM_DECORATIONS.csv",
        "game_data/PROFILE_WEAPON_SKINS.csv",
        "game_data/PROFILE_WEAPON_TRINKETS.csv",
        "game_data/PROFILE_ECHO_THEMES.csv",
        "game_data/PROFILE_ECHO_THEMES_DEFAULTS.csv",
        "game_data/PROFILE_EMOTES.csv",
        "game_data/PROFILE_EMOTES_DEFAULTS.csv",
        "game_data/PROFILE_HEADS.csv",
        "game_data/PROFILE_HEADS_DEFAULTS.csv",
        "game_data/PROFILE_SKINS.csv",
        "game_data/PROFILE_SKINS_DEFAULTS.csv",
        "game_data/BALANCE_NAME_MAPPING.csv",
        "game_data/BALANCE_TO_INV_KEY.csv",
    ];

    let game_data_inputs_array = vec![
        "game_data/VEHICLE_CHASSIS_OUTRUNNER.csv",
        "game_data/VEHICLE_CHASSIS_TECHNICAL.csv",
        "game_data/VEHICLE_CHASSIS_CYCLONE.csv",
        "game_data/VEHICLE_CHASSIS_JETBEAST.csv",
        "game_data/VEHICLE_PARTS_OUTRUNNER.csv",
        "game_data/VEHICLE_PARTS_TECHNICAL.csv",
        "game_data/VEHICLE_PARTS_CYCLONE.csv",
        "game_data/VEHICLE_PARTS_JETBEAST.csv",
        "game_data/VEHICLE_SKINS_OUTRUNNER.csv",
        "game_data/VEHICLE_SKINS_TECHNICAL.csv",
        "game_data/VEHICLE_SKINS_CYCLONE.csv",
        "game_data/VEHICLE_SKINS_JETBEAST.csv",
    ];

    let lootlemon_items = "resources/LOOTLEMON_BL3_ITEMS.csv";

    for input in proto_inputs {
        println!("cargo:rerun-if-changed={}", input);
    }

    for input in &game_data_inputs_kv {
        println!("cargo:rerun-if-changed={}", input);
    }

    for input in &game_data_inputs_array {
        println!("cargo:rerun-if-changed={}", input);
    }

    println!("cargo:rerun-if-changed={}", lootlemon_items);

    let mut all_game_data_inputs = Vec::new();

    for gd in &game_data_inputs_kv {
        all_game_data_inputs.push(gen_game_data_kv(gd));
    }

    for gd in &game_data_inputs_array {
        all_game_data_inputs.push(gen_game_data_array(gd));
    }

    gen_game_data_mod_rs(all_game_data_inputs);

    Codegen::new()
        .out_dir("src/protos")
        .include("protobufs")
        .inputs(proto_inputs)
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .run()
        .expect("Failed to generate protocol buffers");

    //Compression of resources
    let files_to_compress = ["resources/INVENTORY_SERIAL_DB.json"];

    for file in files_to_compress {
        println!("cargo:rerun-if-changed={}", file);
    }

    for file in files_to_compress {
        let mut input_file = std::fs::OpenOptions::new().read(true).open(file).unwrap();
        let output_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}.sz", file))
            .unwrap();

        let mut snappy_wtr = snap::write::FrameEncoder::new(output_file);

        std::io::copy(&mut input_file, &mut snappy_wtr).unwrap();
    }

    let inventory_parts_all_filename = "resources/INVENTORY_PARTS_ALL.csv";

    println!("cargo:rerun-if-changed={}", inventory_parts_all_filename);

    let inventory_parts_info_all_filename = "resources/INVENTORY_PARTS_INFO_ALL.csv";

    println!(
        "cargo:rerun-if-changed={}",
        inventory_parts_info_all_filename
    );

    let inventory_parts_info_all = load_inventory_parts_info_all(inventory_parts_info_all_filename);

    //Generate RON resources
    let inventory_serial_db_json = load_inventory_serial_db_json();

    let inventory_parts_records =
        inventory_parts_all_records(&inventory_serial_db_json, inventory_parts_all_filename);

    //INVENTORY_SERIAL_DB_PARTS_CATEGORIZED
    let inventory_serial_db_categorized_parts = load_inventory_serial_db_parts_categorized(
        &inventory_serial_db_json,
        &inventory_parts_records,
        &inventory_parts_info_all,
    );

    let inventory_serial_db_categorized_parts_ron =
        ron::to_string(&inventory_serial_db_categorized_parts).unwrap();

    //INVENTORY_PARTS_ALL_CATEGORIZED
    let inventory_parts_all =
        load_inventory_all_parts_categorized(inventory_parts_records, &inventory_parts_info_all);
    let inventory_parts_all_categorized_ron = ron::to_string(&inventory_parts_all).unwrap();

    //INVENTORY_BALANCE_PARTS
    let mut inventory_balance_parts = gen_balance_parts(&inventory_serial_db_json);
    inventory_balance_parts.par_sort_by(|a, b| a.short_ident.cmp(&b.short_ident));
    let inventory_balance_parts_ron = ron::to_string(&inventory_balance_parts).unwrap();

    //INVENTORY_INV_DATA_PARTS
    let mut inventory_inv_data_parts = gen_inventory_data_parts(&inventory_serial_db_json);
    inventory_inv_data_parts
        .par_sort_by(|a, b| a.ident.rsplit('.').next().cmp(&b.ident.rsplit('.').next()));
    let inventory_inv_data_parts_ron = ron::to_string(&inventory_inv_data_parts).unwrap();

    //INVENTORY_MANUFACTURER_PARTS
    let mut inventory_manufacturer_parts = gen_manufacturer_parts(&inventory_serial_db_json);
    inventory_manufacturer_parts.par_sort_by(|a, b| a.short_ident.cmp(&b.short_ident));
    let inventory_manufacturer_parts_ron = ron::to_string(&inventory_manufacturer_parts).unwrap();

    //Lootlemon Items
    let all_lootlemon_items = gen_lootlemon_items(lootlemon_items);

    let all_lootlemon_items_ron = ron::to_string(&all_lootlemon_items).unwrap();

    // Compress everything we need
    for (filename, output_data) in [
        (
            "resources/INVENTORY_SERIAL_DB_PARTS_CATEGORIZED",
            inventory_serial_db_categorized_parts_ron,
        ),
        (
            "resources/INVENTORY_PARTS_ALL_CATEGORIZED",
            inventory_parts_all_categorized_ron,
        ),
        (
            "resources/INVENTORY_BALANCE_PARTS",
            inventory_balance_parts_ron,
        ),
        (
            "resources/INVENTORY_INV_DATA_PARTS",
            inventory_inv_data_parts_ron,
        ),
        (
            "resources/INVENTORY_MANUFACTURER_PARTS",
            inventory_manufacturer_parts_ron,
        ),
        ("resources/LOOTLEMON_ITEMS", all_lootlemon_items_ron),
    ] {
        let output_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}.ron.sz", filename))
            .unwrap();

        let mut snappy_wtr = snap::write::FrameEncoder::new(output_file);

        snappy_wtr.write_all(output_data.as_bytes()).unwrap();
    }
}

#[derive(Debug, Deserialize)]
struct GameDataRecord {
    key: String,
    value: String,
}

fn gen_game_data_kv(input_name: &str) -> String {
    let input_array_name = input_name.replace("game_data/", "").replace(".csv", "");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(input_name)
        .unwrap();

    let mut output = String::new();

    let records = rdr
        .deserialize()
        .map(|r| {
            let record: GameDataRecord = r.unwrap();
            record
        })
        .collect::<Vec<_>>();

    writeln!(
        output,
        "pub const {}: [GameDataKv; {}] = [",
        input_array_name,
        records.len()
    )
    .unwrap();

    for record in records {
        writeln!(
            output,
            r#"{:>4}GameDataKv {{ ident: "{}", name: "{}" }},"#,
            " ", record.key, record.value
        )
        .unwrap();
    }

    writeln!(output, "];").unwrap();

    output
}

fn gen_game_data_array(input_name: &str) -> String {
    let input_array_name = input_name.replace("game_data/", "").replace(".csv", "");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(input_name)
        .unwrap();

    let mut output = String::new();

    let records = rdr
        .records()
        .map(|r| {
            let record = r.unwrap();
            let key = record.get(0).expect("Couldn't read key").to_owned();
            key
        })
        .collect::<Vec<_>>();

    writeln!(
        output,
        "pub const {}: [&str; {}] = [",
        input_array_name,
        records.len()
    )
    .unwrap();

    for key in records {
        writeln!(output, r#"{:>4}"{}","#, " ", key).unwrap();
    }

    writeln!(output, "];").unwrap();

    output
}

fn gen_game_data_mod_rs(input_data: Vec<String>) {
    let mut output = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("src/game_data/mod.rs")
        .unwrap();

    writeln!(output, "use std::fmt::Formatter;\n").unwrap();
    writeln!(output, "use anyhow::Result;").unwrap();

    for input in input_data {
        writeln!(output, "{}", input).unwrap();
    }

    writeln!(
        output,
        r#"pub trait GameDataExt {{
    fn get_value_by_key(&self, key: &str) -> Result<&str>;
}}

#[derive(Clone, Copy, Debug, Default, Eq)]
pub struct GameDataKv {{
    pub ident: &'static str,
    pub name: &'static str,
}}

impl std::fmt::Display for GameDataKv {{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{{}}", self.name)
    }}
}}

impl std::cmp::Ord for GameDataKv {{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {{
        self.name.cmp(other.name)
    }}
}}

impl std::cmp::PartialOrd for GameDataKv {{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {{
        Some(self.cmp(other))
    }}
}}

impl std::cmp::PartialEq for GameDataKv {{
    fn eq(&self, other: &Self) -> bool {{
        self.ident == other.ident
    }}
}}"#
    )
    .unwrap();
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct BalancePart {
    pub ident: String,
    pub short_ident: Option<String>,
    pub name: Option<String>,
    pub idx: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct InvDataPart {
    pub ident: String,
    pub idx: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ManufacturerPart {
    pub ident: String,
    pub short_ident: Option<String>,
    pub idx: usize,
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ResourceItem {
    pub manufacturer: String,
    pub rarity: String,
    pub inventory_categorized_parts: Vec<ResourceCategorizedParts>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ResourceCategorizedParts {
    pub category: String,
    pub parts: Vec<ResourcePart>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ResourcePart {
    pub name: String,
    pub min_parts: u8,
    pub max_parts: u8,
    pub dependencies: Option<Vec<String>>,
    pub excluders: Option<Vec<String>>,
    pub info: ResourcePartInfo,
}

#[derive(Debug, Clone, Deserialize)]
struct ResourcePartInfoRecord {
    #[serde(rename = "Part")]
    part: String,
    #[serde(rename = "Positives")]
    positives: Option<String>,
    #[serde(rename = "Negatives")]
    negatives: Option<String>,
    #[serde(rename = "Effects")]
    effects: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ResourcePartInfo {
    pub positives: Option<String>,
    pub negatives: Option<String>,
    pub effects: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct TempHeader {
    manufacturer: String,
    rarity: String,
    balance: String,
}

fn load_inventory_parts_info_all(filename: &str) -> Vec<ResourcePartInfoRecord> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)
        .unwrap();

    rdr.deserialize()
        .map(|r| {
            let record: ResourcePartInfoRecord = r.unwrap();
            record
        })
        .collect::<Vec<_>>()
}

pub fn load_inventory_serial_db_json() -> JsonValue {
    let mut input_file = std::fs::OpenOptions::new()
        .read(true)
        .open("resources/INVENTORY_SERIAL_DB.json")
        .unwrap();

    let mut input_str = String::new();

    input_file.read_to_string(&mut input_str).unwrap();

    json::parse(&input_str).unwrap()
}

fn inventory_parts_all_records(
    inventory_serial_db: &JsonValue,
    input_name: &str,
) -> Vec<ResourceItemRecord> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_name)
        .unwrap();

    let inventory_serial_db_all_parts = inventory_serial_db
        .entries()
        .par_bridge()
        .map(|(category, _)| {
            inventory_serial_db[category]["assets"]
                .members()
                .par_bridge()
                .filter_map(|p| p.to_string().rsplit('.').next().map(|s| s.to_owned()))
                .collect::<HashSet<_>>()
        })
        .flatten()
        .collect::<HashSet<_>>();

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

// This will first get all of the parts found in INVENTORY_SERIAL_DB,
// then get part info from INVENTORY_PARTS_ALL.csv and group accordingly.
fn load_inventory_serial_db_parts_categorized(
    inventory_serial_db: &JsonValue,
    records: &[ResourceItemRecord],
    inventory_parts_info_all: &[ResourcePartInfoRecord],
) -> HashMap<String, Vec<ResourceCategorizedParts>> {
    let records = records;

    inventory_serial_db
        .entries()
        .par_bridge()
        .map(|(inv_db_category, _)| {
            let parts_grouped = inventory_serial_db[inv_db_category]["assets"]
                .members()
                .filter_map(|p| p.to_string().rsplit('.').next().map(|s| s.to_owned()))
                .fold(BTreeMap::new(), |mut curr, inv_db_part_name| {
                    let curr_record = records
                        .par_iter()
                        .find_first(|r| r.part == inv_db_part_name)
                        .map(|r| r.to_owned());

                    let part_info = inventory_parts_info_all
                        .par_iter()
                        .find_first(|i| i.part == inv_db_part_name);

                    let info = if let Some(part_info) = part_info {
                        ResourcePartInfo {
                            positives: part_info.positives.clone(),
                            negatives: part_info.negatives.clone(),
                            effects: part_info.effects.clone(),
                        }
                    } else {
                        ResourcePartInfo {
                            positives: None,
                            negatives: None,
                            effects: None,
                        }
                    };

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
                            info,
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
                            info,
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

fn load_inventory_all_parts_categorized(
    records: Vec<ResourceItemRecord>,
    inventory_parts_info_all: &[ResourcePartInfoRecord],
) -> HashMap<String, ResourceItem> {
    let parts_grouped = records
        .into_iter()
        .fold(BTreeMap::new(), |mut curr, inv_part| {
            let inventory_part_header = TempHeader {
                manufacturer: inv_part.manufacturer,
                rarity: inv_part.rarity,
                balance: inv_part.balance,
            };

            let inv_part_s = inv_part.part.as_str();

            let part_info = inventory_parts_info_all
                .par_iter()
                .find_first(|i| i.part == inv_part_s);

            let info = if let Some(part_info) = part_info {
                ResourcePartInfo {
                    positives: part_info.positives.clone(),
                    negatives: part_info.negatives.clone(),
                    effects: part_info.effects.clone(),
                }
            } else {
                ResourcePartInfo {
                    positives: None,
                    negatives: None,
                    effects: None,
                }
            };

            let inventory_part = ResourcePart {
                name: inv_part.part,
                min_parts: inv_part.min_parts,
                max_parts: inv_part.max_parts,
                dependencies: inv_part.dependencies,
                excluders: inv_part.excluders,
                info,
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
                            info: p.info.clone(),
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

fn gen_balance_parts(inventory_serial_db_json: &JsonValue) -> Vec<BalancePart> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("game_data/BALANCE_NAME_MAPPING.csv")
        .unwrap();

    let balance_name_mappings = rdr
        .deserialize()
        .map(|r| {
            let record: GameDataRecord = r.unwrap();
            record
        })
        .collect::<Vec<_>>();

    inventory_serial_db_json["InventoryBalanceData"]["assets"]
        .members()
        .enumerate()
        .par_bridge()
        .map(|(i, part)| {
            let ident = part.to_string();
            let short_ident = ident.rsplit('.').next().map(|s| s.to_owned());
            let name = balance_name_mappings
                .par_iter()
                .find_first(|gd| ident.to_lowercase().contains(&gd.key))
                .map(|gd| gd.value.to_owned());

            BalancePart {
                ident,
                short_ident,
                name,
                idx: i + 1,
            }
        })
        .collect::<Vec<_>>()
}

fn gen_inventory_data_parts(inventory_serial_db_json: &JsonValue) -> Vec<InvDataPart> {
    inventory_serial_db_json["InventoryData"]["assets"]
        .members()
        .enumerate()
        .par_bridge()
        .map(|(i, part)| {
            let ident = part.to_string();

            InvDataPart { ident, idx: i + 1 }
        })
        .collect::<Vec<_>>()
}

fn gen_manufacturer_parts(inventory_serial_db_json: &JsonValue) -> Vec<ManufacturerPart> {
    inventory_serial_db_json["ManufacturerData"]["assets"]
        .members()
        .enumerate()
        .par_bridge()
        .map(|(i, part)| {
            let ident = part.to_string();
            let short_ident = ident.rsplit('.').next().map(|s| s.to_owned());

            ManufacturerPart {
                ident,
                short_ident,
                idx: i + 1,
            }
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Serialize)]
pub struct LootlemonItem {
    pub serial: String,
    pub link: String,
}

impl LootlemonItem {
    pub fn from_record(record: StringRecord) -> Self {
        Self {
            serial: record.get(1).unwrap().trim().to_owned(),
            link: record.get(2).unwrap().trim().to_owned(),
        }
    }
}

pub fn gen_lootlemon_items(input_name: &str) -> Vec<LootlemonItem> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_name)
        .unwrap();

    rdr.records()
        .into_iter()
        .map(|r| LootlemonItem::from_record(r.unwrap()))
        .collect()
}
