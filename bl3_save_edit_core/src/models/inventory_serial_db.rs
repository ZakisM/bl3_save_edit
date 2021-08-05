use std::convert::TryInto;
use std::io::Read;

use anyhow::{bail, Context, Result};
use json::JsonValue;

use crate::resources::INVENTORY_SERIAL_DB_JSON_COMPRESSED;

pub struct InventorySerialDb {
    pub data: JsonValue,
    pub max_version: usize,
}

impl InventorySerialDb {
    pub fn load() -> Result<Self> {
        let mut rdr = snap::read::FrameDecoder::new(INVENTORY_SERIAL_DB_JSON_COMPRESSED);

        let mut decompressed_bytes = String::new();

        rdr.read_to_string(&mut decompressed_bytes)
            .context("failed to read decompressed bytes")?;

        let data = json::parse(&decompressed_bytes)?;

        let max_version = data
            .entries()
            .into_iter()
            .map(|(i, _)| {
                data[i]["versions"]
                    .members()
                    .map(|c| c["version"].as_isize())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .flatten()
            .max()
            .and_then(|v| v.try_into().ok())
            .context("failed to read inventory serial db max version")?;

        Ok(Self { data, max_version })
    }

    pub fn get_num_bits(&self, category: &str, version: usize) -> Result<usize> {
        let mut cur_bits = self.data[category]["versions"][0]["bits"]
            .as_isize()
            .context("failed to read cur_bits")?;

        let version_isize = version as isize;

        for cat_version in self.data[category]["versions"].members() {
            let category_version = cat_version["version"]
                .as_isize()
                .context("category version was missing")?;

            if category_version > version_isize {
                return Ok(cur_bits as usize);
            } else if version_isize >= category_version {
                cur_bits = cat_version["bits"]
                    .as_isize()
                    .context("category bits was missing")?;
            }
        }

        Ok(cur_bits as usize)
    }

    pub fn get_part(&self, category: &str, index: usize) -> Result<String> {
        let assets = self.data[category]["assets"].members();

        if index > assets.len() {
            bail!("index was greater than assets length")
        } else {
            Ok(self.data[category]["assets"][index - 1].to_string())
        }
    }
}
