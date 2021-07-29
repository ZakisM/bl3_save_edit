use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as Write2;
use std::fs::OpenOptions;
use std::io::Write;

use protobuf_codegen_pure::{Codegen, Customize};
use serde::Deserialize;

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
    let game_data_inventory_parts = vec!["game_data/INVENTORY_PARTS_SHIELDS.csv"];

    for input in proto_inputs {
        println!("cargo:rerun-if-changed={}", input);
    }

    for input in &game_data_inputs_kv {
        println!("cargo:rerun-if-changed={}", input);
    }

    for input in &game_data_inputs_array {
        println!("cargo:rerun-if-changed={}", input);
    }

    for input in &game_data_inventory_parts {
        println!("cargo:rerun-if-changed={}", input);
    }

    let mut all_game_data_inputs = Vec::new();

    for gd in &game_data_inputs_kv {
        all_game_data_inputs.push(gen_game_data_kv(gd));
    }

    for gd in &game_data_inputs_array {
        all_game_data_inputs.push(gen_game_data_array(gd));
    }

    for gd in &game_data_inventory_parts {
        all_game_data_inputs.push(gen_inventory_parts(gd));
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
}

#[derive(Debug, serde::Deserialize)]
struct InventoryPartRecord {
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct InventoryPartHeader {
    manufacturer: String,
    rarity: String,
    balance: String,
}

type InventoryPartBody = BTreeMap<String, BTreeSet<InventoryPart>>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, serde::Serialize)]
struct InventoryPart {
    name: String,
    min_parts: u8,
    max_parts: u8,
    dependencies: Option<Vec<String>>,
    excluders: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct GameDataRecord {
    key: String,
    value: String,
}

fn gen_inventory_parts(input_name: &str) -> String {
    let input_array_name = input_name.replace("game_data/", "").replace(".csv", "");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_name)
        .unwrap();

    let mut output = String::new();

    let records = rdr
        .deserialize()
        .map(|r| {
            let mut record: InventoryPartRecord = r.unwrap();

            if let Some(curr_dependencies) = &record.dependencies {
                let all_dependencies = curr_dependencies
                    .get(0)
                    .unwrap()
                    .split(',')
                    .map(|s| s.trim().to_owned())
                    .collect::<Vec<_>>();

                record.dependencies = Some(all_dependencies);
            };

            if let Some(curr_excluders) = &record.excluders {
                let all_excluders = curr_excluders
                    .get(0)
                    .unwrap()
                    .split(',')
                    .map(|s| s.trim().to_owned())
                    .collect::<Vec<_>>();

                record.excluders = Some(all_excluders);
            };

            record
        })
        .collect::<Vec<_>>();

    let mut parts_grouped: BTreeMap<InventoryPartHeader, InventoryPartBody> = BTreeMap::new();

    for inv_part in records {
        let inventory_part_header = InventoryPartHeader {
            manufacturer: inv_part.manufacturer,
            rarity: inv_part.rarity,
            balance: inv_part.balance,
        };

        let inventory_part = InventoryPart {
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
    }

    writeln!(
        output,
        r"pub static {}: Lazy<HashMap<Cow<str>, InventoryPartBody>> = Lazy::new(|| {{
    let mut m = HashMap::new();",
        input_array_name
    )
    .unwrap();

    for (header, body) in parts_grouped {
        writeln!(output, "{:>4}let parts = vec![", " ").unwrap();

        for (category, parts) in body {
            writeln!(output, "{:>8}InventoryPart {{", " ").unwrap();
            writeln!(output, r#"{:>12}category: "{}","#, " ", category).unwrap();
            writeln!(output, "{:>12}parts: vec![", " ").unwrap();

            for part in parts {
                writeln!(output, "{:>16}Part {{", " ").unwrap();

                writeln!(output, r#"{:>20}name: "{}","#, " ", part.name).unwrap();
                writeln!(output, "{:>20}min_parts: {},", " ", part.min_parts).unwrap();
                writeln!(output, "{:>20}max_parts: {},", " ", part.max_parts).unwrap();

                if let Some(dependencies) = part.dependencies {
                    writeln!(output, "{:>20}dependencies: Some(vec![", " ").unwrap();

                    for dependency in dependencies {
                        writeln!(output, r#"{:>24}"{}","#, " ", dependency).unwrap();
                    }

                    writeln!(output, "{:>20}]),", " ").unwrap();
                } else {
                    writeln!(output, "{:>20}dependencies: None,", " ").unwrap();
                }

                if let Some(excluders) = part.excluders {
                    writeln!(output, "{:>20}excluders: Some(vec![", " ").unwrap();

                    for excluder in excluders {
                        writeln!(output, r#"{:>24}"{}","#, " ", excluder).unwrap();
                    }

                    writeln!(output, "{:>20}]),", " ").unwrap();
                } else {
                    writeln!(output, "{:>20}excluders: None,", " ").unwrap();
                }

                writeln!(output, "{:>16}}},", " ").unwrap();
            }

            writeln!(output, "{:>12}]", " ").unwrap();
            writeln!(output, "{:>8}}},", " ").unwrap();
        }

        writeln!(output, "{:>4}];", " ").unwrap();

        writeln!(
            output,
            r#"{:>4}m.insert(Cow::from("{}"), InventoryPartBody {{ manufacturer: "{}", rarity: "{}", parts }});"#,
            " ", header.balance, header.manufacturer, header.rarity
        )
            .unwrap();
    }

    writeln!(output, "m").unwrap();

    writeln!(output, "}});").unwrap();

    output
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

    writeln!(output, "use std::collections::HashMap;").unwrap();
    writeln!(output, "use std::fmt::Formatter;\n").unwrap();
    writeln!(output, "use anyhow::Result;").unwrap();
    writeln!(output, "use once_cell::sync::Lazy;").unwrap();
    writeln!(output, "use std::borrow::Cow;\n").unwrap();

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

    writeln!(
        output,
        r#"
        #[derive(Debug)]
pub struct InventoryPartBody {{
    pub manufacturer: &'static str,
    pub rarity: &'static str,
    pub parts: Vec<InventoryPart>,
}}

#[derive(Debug)]
pub struct InventoryPart {{
    pub category: &'static str,
    pub parts: Vec<Part>,
}}

#[derive(Debug)]
pub struct Part {{
    pub name: &'static str,
    pub min_parts: u8,
    pub max_parts: u8,
    pub dependencies: Option<Vec<&'static str>>,
    pub excluders: Option<Vec<&'static str>>,
}}"#
    )
    .unwrap();
}
