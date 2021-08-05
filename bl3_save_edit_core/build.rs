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

    for input in proto_inputs {
        println!("cargo:rerun-if-changed={}", input);
    }

    for input in &game_data_inputs_kv {
        println!("cargo:rerun-if-changed={}", input);
    }

    for input in &game_data_inputs_array {
        println!("cargo:rerun-if-changed={}", input);
    }

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
    let files_to_compress = [
        "resources/INVENTORY_PARTS_ALL.csv",
        "resources/INVENTORY_SERIAL_DB.json",
    ];

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
