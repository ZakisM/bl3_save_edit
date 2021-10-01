use std::fmt::Formatter;
use std::str::FromStr;

use anyhow::{bail, Context, Result};
use bitflags::bitflags;
use bitvec::prelude::*;
use byteorder::{BigEndian, WriteBytesExt};
use encoding_rs::mem::decode_latin1;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Deserialize;
use strum::{Display, EnumString};
use tracing::warn;

use crate::arbitrary_bits::{ArbitraryBitVec, ArbitraryBits};
use crate::game_data::{BALANCE_NAME_MAPPING, BALANCE_TO_INV_KEY};
use crate::parser::read_be_signed_int;
use crate::resources::{
    INVENTORY_INV_DATA_PARTS, INVENTORY_PARTS_ALL_CATEGORIZED, INVENTORY_SERIAL_DB,
    INVENTORY_SERIAL_DB_PARTS_CATEGORIZED,
};

pub const MAX_BL3_ITEM_PARTS: usize = 63;
pub const MAX_BL3_ITEM_ANOINTMENTS: usize = 15;

bitflags! {
    pub struct ItemFlags: i32 {
        const SEEN = 0x1;
        const FAVORITE = 0x2;
        const JUNK = 0x4;
    }
}

// Translated from https://github.com/apocalyptech/bl3-cli-saveedit/blob/master/bl3save/datalib.py
// All credits to apocalyptech

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bl3Item {
    pub serial_version: u8,
    pub orig_seed: i32,
    decrypted_serial: Vec<u8>,
    pub data_version: usize,
    pub balance_bits: usize,
    balance_part: BalancePart,
    pub inv_data_bits: usize,
    inv_data_part: InvDataPart,
    pub manufacturer_bits: usize,
    manufacturer_part: ManufacturerPart,
    level: usize,
    pub item_parts: Option<Bl3ItemParts>,
    pub item_type: ItemType,
    pub flags: Option<ItemFlags>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bl3ItemParts {
    pub part_inv_key: String,
    pub part_bits: usize,
    parts: Vec<Bl3Part>,
    pub generic_part_bits: usize,
    generic_parts: Vec<Bl3Part>,
    pub additional_data: Vec<usize>,
    pub num_customs: usize,
    pub rerolled: usize,
    pub rarity: ItemRarity,
    pub weapon_type: Option<WeaponType>,
}

impl Bl3ItemParts {
    pub fn parts(&self) -> &Vec<Bl3Part> {
        &self.parts
    }

    pub fn generic_parts(&self) -> &Vec<Bl3Part> {
        &self.generic_parts
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Deserialize)]
pub struct BalancePart {
    pub ident: String,
    pub short_ident: Option<String>,
    pub name: Option<String>,
    pub idx: usize,
}

impl std::fmt::Display for BalancePart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.ident.rsplit('/').next().unwrap_or(&self.ident)
        )?;

        if let Some(name) = &self.name {
            write!(f, " - ({})", name)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Deserialize)]
pub struct InvDataPart {
    pub ident: String,
    pub idx: usize,
}

impl std::fmt::Display for InvDataPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.ident.rsplit('/').next().unwrap_or(&self.ident)
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Deserialize)]
pub struct ManufacturerPart {
    pub ident: String,
    pub short_ident: Option<String>,
    pub idx: usize,
}

impl std::fmt::Display for ManufacturerPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.ident.rsplit('/').next().unwrap_or(&self.ident)
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bl3Part {
    pub ident: String,
    pub short_ident: Option<String>,
    pub idx: usize,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Display, EnumString)]
pub enum ItemType {
    #[strum(serialize = "BPInvPart_Artifact_C", to_string = "Artifact")]
    Artifact,
    #[strum(serialize = "BPInvPart_ClassMod_C", to_string = "Class Mod")]
    ClassMod,
    #[strum(serialize = "BPInvPart_GrenadeMod_C", to_string = "Grenade Mod")]
    GrenadeMod,
    #[strum(serialize = "BPInvPart_Shield_C", to_string = "Shield")]
    Shield,
    #[strum(to_string = "Weapon")]
    Weapon,
    #[strum(to_string = "Other")]
    Other,
}

impl std::default::Default for ItemType {
    fn default() -> Self {
        Self::Other
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Display, EnumString)]
pub enum ItemRarity {
    #[strum(
        serialize = "01/Common",
        serialize = "01/Common (Starting Gear)",
        to_string = "Common"
    )]
    Common,
    #[strum(serialize = "02/Uncommon", to_string = "Uncommon")]
    Uncommon,
    #[strum(
        serialize = "03/Rare",
        serialize = "03/Rare E-Tech",
        to_string = "Rare"
    )]
    Rare,
    #[strum(
        serialize = "04/Very Rare",
        serialize = "04/Very Rare E-Tech",
        to_string = "Very Rare"
    )]
    VeryRare,
    #[strum(serialize = "05/Legendary", to_string = "Legendary")]
    Legendary,
    #[strum(serialize = "Named Weapon", to_string = "Unique Weapon")]
    NamedWeapon,
    Unknown,
}

impl std::default::Default for ItemRarity {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Display)]
pub enum WeaponType {
    #[strum(to_string = "Pistol")]
    Pistol,
    #[strum(to_string = "Shotgun")]
    Shotgun,
    #[strum(to_string = "SMG")]
    Smg,
    #[strum(to_string = "Assault Rifle")]
    Ar,
    #[strum(to_string = "Sniper")]
    Sniper,
    #[strum(to_string = "Heavy")]
    Heavy,
}

impl Bl3Item {
    pub fn from_serial_bytes(serial: &[u8], flags: Option<ItemFlags>) -> Result<Self> {
        let serial = serial;

        if serial.len() < 5 {
            bail!("Serial length must be longer than 4 characters.");
        }

        let initial_byte = serial[0];

        if initial_byte != 3 && initial_byte != 4 {
            bail!("Serial version was not 3 or 4 so we do not know how to decrypt this item.");
        }

        let serial_version = initial_byte;

        let orig_seed = read_be_signed_int(&serial[1..5])?.1;

        let mut serial = serial.to_vec();

        let decrypted_serial = Self::bogodecrypt(&mut serial[5..], orig_seed);

        let orig_checksum = &decrypted_serial[..2];

        let data_to_checksum = [&serial[..5], b"\xFF\xFF", &decrypted_serial[2..]].concat();

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&data_to_checksum);
        let computed_crc = hasher.finalize();

        let mut computed_checksum = Vec::with_capacity(2);

        computed_checksum
            .write_u16::<BigEndian>((((computed_crc >> 16) ^ computed_crc) & 0xFFFF) as u16)?;

        if orig_checksum != computed_checksum {
            bail!("The expected checksum when deserializing this item does not match the original checksum");
        }

        // What we will actually store
        let decrypted_serial = &decrypted_serial[2..];

        // parse the serial data
        let mut bits = ArbitraryBits::new(decrypted_serial.view_bits::<Lsb0>());

        let ident = bits.eat(8)?;

        // Ident will be 0 if is item is not obfuscated
        if ident != 128 && ident != 0 {
            bail!(
                "The 'ident' header of this item should be 128 or 0, but instead it is: {}",
                ident
            )
        }

        let data_version = bits.eat(7)?;

        if data_version > INVENTORY_SERIAL_DB.max_version {
            bail!("Cannot parse item as it is newer than the version of this item parser, expected: {}, found: {}", INVENTORY_SERIAL_DB.max_version, data_version);
        }

        let (balance, balance_bits, balance_idx) =
            Self::inv_db_header_part("InventoryBalanceData", &mut bits, data_version)?;

        let (inv_data, inv_data_bits, inv_data_idx) =
            Self::inv_db_header_part("InventoryData", &mut bits, data_version)?;

        let (manufacturer, manufacturer_bits, manufacturer_idx) =
            Self::inv_db_header_part("ManufacturerData", &mut bits, data_version)?;

        let manufacturer_short = manufacturer.rsplit('.').next().map(|s| s.to_owned());

        let level = bits.eat(7)?;

        let balance_short_name = balance.rsplit('.').next().map(|s| s.to_owned());

        let item_part_data = &INVENTORY_PARTS_ALL_CATEGORIZED;
        let item_part_info = balance_short_name
            .as_ref()
            .and_then(|bs| item_part_data.get(bs));

        let balance_lower = balance.to_lowercase();
        let balance_short_name_lower = balance_short_name
            .as_ref()
            .map(|b| b.to_lowercase())
            .unwrap_or_else(|| balance_lower.clone());

        let balance_eng_name = BALANCE_NAME_MAPPING
            .par_iter()
            .find_first(|gd| {
                balance_short_name_lower == gd.ident.rsplit('/').next().unwrap_or(gd.ident)
            })
            .map(|gd| gd.name.to_owned());

        let balance_part = BalancePart {
            ident: balance.clone(),
            short_ident: balance_short_name,
            name: balance_eng_name,
            idx: balance_idx,
        };

        let inv_data_part = InvDataPart {
            ident: inv_data,
            idx: inv_data_idx,
        };

        let manufacturer_part = ManufacturerPart {
            ident: manufacturer,
            short_ident: manufacturer_short,
            idx: manufacturer_idx,
        };

        let mut item_type = ItemType::Other;

        let item_parts = if let Some(part_inv_key) = BALANCE_TO_INV_KEY
            .par_iter()
            .find_first(|gd| balance_lower == gd.ident)
            .map(|gd| gd.name.to_owned())
        {
            let mut should_not_allow_parts_parsing = false;

            let (part_bits, parts) =
                Self::inv_db_header_part_repeated(&part_inv_key, &mut bits, data_version, 6)?;

            //generics (anointment + mayhem)
            let (generic_part_bits, generic_parts) = Self::inv_db_header_part_repeated(
                "InventoryGenericPartData",
                &mut bits,
                data_version,
                4,
            )?;

            let additional_count = bits.eat(8)?;

            let additional_data = (0..additional_count)
                .map(|_| bits.eat(8))
                .collect::<Result<Vec<_>>>()?;

            let num_customs = bits.eat(4)?;

            if num_customs != 0 {
                warn!(
                    "Number of customs should be 0 for this item but it is: {}",
                    num_customs
                );
                should_not_allow_parts_parsing = true;
            }

            let rerolled = if serial_version >= 4 { bits.eat(8)? } else { 0 };

            if bits.len() > 7 {
                warn!("Remaining data length for item was more than expected. Expected length of 7 or less but found length of: {}.", bits.len());
            }

            if bits.bitslice().count_ones() > 0 {
                bail!("Could not fully parse the item data, there was unexpected data left.")
            }

            let rarity = item_part_info
                .and_then(|info| ItemRarity::from_str(&info.rarity).ok())
                .unwrap_or_default();

            let weapon_type = match &balance {
                b if b.contains("_PS_") => Some(WeaponType::Pistol),
                b if b.contains("_SG_") => Some(WeaponType::Shotgun),
                b if b.contains("_SM_") => Some(WeaponType::Smg),
                b if b.contains("_AR_") => Some(WeaponType::Ar),
                b if b.contains("_SR_") => Some(WeaponType::Sniper),
                b if b.contains("_HW_") => Some(WeaponType::Heavy),
                _ => None,
            };

            item_type = if weapon_type.is_some() {
                ItemType::Weapon
            } else {
                ItemType::from_str(&part_inv_key).unwrap_or_default()
            };

            let item_parts = Bl3ItemParts {
                part_inv_key,
                part_bits,
                parts,
                generic_part_bits,
                generic_parts,
                additional_data,
                num_customs,
                rerolled,
                rarity,
                weapon_type,
            };

            if should_not_allow_parts_parsing {
                None
            } else {
                Some(item_parts)
            }
        } else {
            None
        };

        let decrypted_serial = decrypted_serial.to_vec();

        Ok(Self {
            serial_version,
            orig_seed,
            decrypted_serial,
            data_version,
            balance_bits,
            balance_part,
            inv_data_bits,
            inv_data_part,
            manufacturer_bits,
            manufacturer_part,
            level,
            item_parts,
            item_type,
            flags,
        })
    }

    pub fn from_serial_base64(serial: &str) -> Result<Self> {
        if serial.len() < 5 {
            bail!("Serial length must be longer than 4 characters.");
        }

        let serial_start = serial[0..4].to_lowercase();

        if serial_start != "bl3(" || !serial.ends_with(')') {
            bail!("Serial must start with 'BL3(' and end with ')'.")
        }

        let decoded = base64::decode(&serial[4..serial.len() - 1])?;

        Self::from_serial_bytes(&decoded, None)
    }

    pub fn encrypt_serial(&self, seed: i32) -> Result<Vec<u8>> {
        let mut header = Vec::new();
        header.write_u8(self.serial_version)?;
        header.write_i32::<BigEndian>(seed)?;

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&header);
        hasher.update(b"\xFF\xFF");
        hasher.update(&self.decrypted_serial);

        let crc32 = hasher.finalize();

        let mut checksum = Vec::new();

        checksum.write_u16::<BigEndian>((((crc32 >> 16) ^ crc32) & 0xFFFF) as u16)?;

        let mut data: Vec<u8> = [checksum, self.decrypted_serial.to_vec()].concat();

        let encrypted = Self::bogoencrypt(&mut data, seed);

        let encrypted_full = [header, encrypted].concat();

        Ok(encrypted_full)
    }

    pub fn get_serial_number(&self, orig_seed: bool) -> Result<Vec<u8>> {
        let seed = if orig_seed { self.orig_seed } else { 0 };

        self.encrypt_serial(seed)
    }

    pub fn get_serial_number_base64(&self, orig_seed: bool) -> Result<String> {
        let serial = self.get_serial_number(orig_seed)?;

        let encoded = base64::encode(serial);

        let non_latin = format!("BL3({})", encoded);

        let res_bytes = decode_latin1(non_latin.as_bytes());

        let res = res_bytes.to_string();

        Ok(res)
    }

    pub fn balance_part(&self) -> &BalancePart {
        &self.balance_part
    }

    pub fn inv_data_part(&self) -> &InvDataPart {
        &self.inv_data_part
    }

    pub fn manufacturer_part(&self) -> &ManufacturerPart {
        &self.manufacturer_part
    }

    pub fn set_balance(&mut self, balance_part: BalancePart) -> Result<()> {
        let balance_ident_lower = balance_part.ident.to_lowercase();

        match BALANCE_TO_INV_KEY
            .iter()
            .find(|gd| balance_ident_lower == gd.ident)
            .map(|gd| gd.name.to_owned())
        {
            None => {
                warn!(
                    "set_balance error: no part_inv_key found for: {}",
                    balance_part.ident
                );

                self.item_parts = None;
            }
            Some(part_inv_key) => {
                if let Some(item_parts) = &mut self.item_parts {
                    let inventory_serial_db_parts_categorized =
                        &*INVENTORY_SERIAL_DB_PARTS_CATEGORIZED;

                    let all_parts_list = inventory_serial_db_parts_categorized.get(&part_inv_key);

                    item_parts.part_inv_key = part_inv_key;

                    // Try to keep valid parts on our item instead of removing all
                    if let Some(all_parts) = all_parts_list {
                        item_parts.parts.retain(|p| {
                            all_parts.par_iter().any(|rcp| {
                                rcp.parts.par_iter().any(|rp| {
                                    if let Some(short_ident) = &p.short_ident {
                                        rp.name == *short_ident
                                    } else {
                                        false
                                    }
                                })
                            })
                        });
                    } else {
                        item_parts.parts = Vec::new();
                    }
                } else {
                    self.item_parts = Some(Bl3ItemParts {
                        part_inv_key,
                        ..Bl3ItemParts::default()
                    })
                }
            }
        }

        self.balance_part = balance_part;

        //try to find a matching manufacturer_part
        if let Some(short_ident) = &self.balance_part.short_ident {
            let short_ident_s = short_ident.replace("InvBal", "");

            if let Some(inv_data_part) = INVENTORY_INV_DATA_PARTS
                .iter()
                .find(|inv_part| inv_part.ident.contains(&short_ident_s))
            {
                self.inv_data_part = inv_data_part.to_owned();
            }
        }

        self.update_weapon_serial()?;

        Ok(())
    }

    pub fn set_inv_data(&mut self, inv_data_part: InvDataPart) -> Result<()> {
        self.inv_data_part = inv_data_part;

        self.update_weapon_serial()?;

        Ok(())
    }

    pub fn set_manufacturer(&mut self, manufacturer_part: ManufacturerPart) -> Result<()> {
        self.manufacturer_part = manufacturer_part;

        self.update_weapon_serial()?;

        Ok(())
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn set_level(&mut self, new_level: usize) -> Result<()> {
        self.level = new_level;

        self.update_weapon_serial()?;

        Ok(())
    }

    pub fn remove_part(&mut self, part: &Bl3Part) -> Result<()> {
        if let Some(item_parts) = &mut self.item_parts {
            if let Some(part_index) = item_parts
                .parts
                .iter_mut()
                .position(|p| p.ident == part.ident)
            {
                item_parts.parts.remove(part_index);
            }

            self.update_weapon_serial()?;
        }

        Ok(())
    }

    pub fn add_part(&mut self, part: Bl3Part) -> Result<()> {
        if let Some(item_parts) = &mut self.item_parts {
            item_parts.parts.push(part);

            self.update_weapon_serial()?;
        }

        Ok(())
    }

    pub fn remove_generic_part(&mut self, part: &Bl3Part) -> Result<()> {
        if let Some(item_parts) = &mut self.item_parts {
            if let Some(part_index) = item_parts
                .generic_parts
                .iter_mut()
                .position(|p| p.ident == part.ident)
            {
                item_parts.generic_parts.remove(part_index);
            }

            self.update_weapon_serial()?;
        }

        Ok(())
    }

    pub fn add_generic_part(&mut self, part: Bl3Part) -> Result<()> {
        if let Some(item_parts) = &mut self.item_parts {
            item_parts.generic_parts.push(part);

            self.update_weapon_serial()?;
        }

        Ok(())
    }

    pub fn move_part_up(&mut self, index: &mut usize) -> Result<()> {
        let curr_index = *index;

        if let Some(item_parts) = &mut self.item_parts {
            if curr_index != 0 {
                let parts = &mut item_parts.parts;

                let current = parts
                    .get(curr_index)
                    .context("failed to find this part on the item")?
                    .to_owned();

                parts.remove(curr_index);
                parts.insert(curr_index - 1, current);

                *index -= 1;
            }
        }

        self.update_weapon_serial()?;

        Ok(())
    }

    pub fn move_part_down(&mut self, index: &mut usize) -> Result<()> {
        let curr_index = *index;

        if let Some(item_parts) = &mut self.item_parts {
            if curr_index != item_parts.parts.len() - 1 {
                let parts = &mut item_parts.parts;

                let current = parts
                    .get(curr_index)
                    .context("failed to find this part on the item")?
                    .to_owned();

                parts.remove(curr_index);
                parts.insert(curr_index + 1, current);

                *index += 1;
            }
        }

        self.update_weapon_serial()?;

        Ok(())
    }

    pub fn move_part_top(&mut self, index: &mut usize) -> Result<()> {
        let curr_index = *index;

        if let Some(item_parts) = &mut self.item_parts {
            if curr_index != 0 {
                let parts = &mut item_parts.parts;

                let current = parts
                    .get(curr_index)
                    .context("failed to find this part on the item")?
                    .to_owned();

                parts.remove(curr_index);
                parts.insert(0, current);

                *index = 0;
            }
        }

        self.update_weapon_serial()?;

        Ok(())
    }

    pub fn move_part_bottom(&mut self, index: &mut usize) -> Result<()> {
        let curr_index = *index;

        if let Some(item_parts) = &mut self.item_parts {
            let len = item_parts.parts.len() - 1;

            if curr_index != len {
                let parts = &mut item_parts.parts;

                let current = parts
                    .get(curr_index)
                    .context("failed to find this part on the item")?
                    .to_owned();

                parts.remove(curr_index);
                parts.insert(len, current);

                *index = len;
            }
        }

        self.update_weapon_serial()?;

        Ok(())
    }

    pub fn update_weapon_serial(&mut self) -> Result<()> {
        let serial_db = &*INVENTORY_SERIAL_DB;

        self.data_version = serial_db.max_version;
        self.balance_bits = serial_db.get_num_bits("InventoryBalanceData", self.data_version)?;
        self.inv_data_bits = serial_db.get_num_bits("InventoryData", self.data_version)?;
        self.manufacturer_bits = serial_db.get_num_bits("ManufacturerData", self.data_version)?;

        let mut new_serial_bits = ArbitraryBitVec::<Lsb0, u8>::new();

        // Header
        new_serial_bits.append_le(128, 8);
        new_serial_bits.append_le(self.data_version, 7);
        new_serial_bits.append_le(self.balance_part.idx, self.balance_bits);
        new_serial_bits.append_le(self.inv_data_part.idx, self.inv_data_bits);
        new_serial_bits.append_le(self.manufacturer_part.idx, self.manufacturer_bits);
        new_serial_bits.append_le(self.level, 7);

        if let Some(item_parts) = &mut self.item_parts {
            item_parts.part_bits =
                serial_db.get_num_bits(&item_parts.part_inv_key, self.data_version)?;
            item_parts.generic_part_bits =
                serial_db.get_num_bits("InventoryGenericPartData", self.data_version)?;

            // Parts
            new_serial_bits.append_le(item_parts.parts.len(), 6);

            item_parts.parts.iter().for_each(|p| {
                new_serial_bits.append_le(p.idx, item_parts.part_bits);
            });

            // Generics
            new_serial_bits.append_le(item_parts.generic_parts.len(), 4);

            item_parts.generic_parts.iter().for_each(|gp| {
                new_serial_bits.append_le(gp.idx, item_parts.generic_part_bits);
            });

            // Additional data
            new_serial_bits.append_le(item_parts.additional_data.len(), 8);

            item_parts.additional_data.iter().for_each(|a| {
                new_serial_bits.append_le(*a, 8);
            });

            new_serial_bits.append_le(item_parts.num_customs, 4);

            if self.serial_version >= 4 {
                new_serial_bits.append_le(item_parts.rerolled, 8);
            }
        }

        let new_decrypted_serial = new_serial_bits.bitvec.into_vec();

        self.decrypted_serial = new_decrypted_serial;

        let full_serial = self.encrypt_serial(0)?;

        *self = Bl3Item::from_serial_bytes(&full_serial, self.flags)?;

        Ok(())
    }

    fn xor_data(data: &mut [u8], seed: i32) {
        if seed != 0 {
            let mut xor = ((seed >> 5) as i64) & 0xFFFFFFFF;

            data.iter_mut().for_each(|d| {
                xor = (xor * 0x10A860C1) % 0xFFFFFFFB;
                *d ^= xor as u8;
            });
        }
    }

    fn bogoencrypt(data: &mut [u8], seed: i32) -> Vec<u8> {
        let data_len = data.len();

        let steps = (seed & 0x1F) as usize % (data_len);

        let first_half = &data[steps..];
        let second_half = &data[..steps];

        let mut data: Vec<u8> = [first_half, second_half].concat();

        Self::xor_data(&mut data, seed);

        data
    }

    fn bogodecrypt(data: &mut [u8], seed: i32) -> Vec<u8> {
        Self::xor_data(data, seed);

        let data_len = data.len();

        let steps = (seed & 0x1F) as usize % (data_len);

        let first_half = &data[data_len - steps..];
        let second_half = &data[..data_len - steps];

        [first_half, second_half].concat()
    }

    fn inv_db_header_part(
        category: &str,
        bits: &mut ArbitraryBits,
        version: usize,
    ) -> Result<(String, usize, usize)> {
        let num_bits = INVENTORY_SERIAL_DB.get_num_bits(category, version)?;

        let part_idx = bits.eat(num_bits)?;

        let part = INVENTORY_SERIAL_DB
            .get_part_ident(category, part_idx)
            .unwrap_or_else(|_| "unknown".to_owned());

        Ok((part, num_bits, part_idx))
    }

    fn inv_db_header_part_repeated(
        category: &str,
        bits: &mut ArbitraryBits,
        version: usize,
        count_bits: usize,
    ) -> Result<(usize, Vec<Bl3Part>)> {
        let num_bits = INVENTORY_SERIAL_DB.get_num_bits(category, version)?;
        let num_parts = bits.eat(count_bits)?;

        let mut parts = Vec::with_capacity(num_parts);

        for _ in 0..num_parts {
            let part_idx = bits.eat(num_bits)?;

            let ident = INVENTORY_SERIAL_DB
                .get_part_ident(category, part_idx)
                .unwrap_or_else(|_| "unknown".to_owned());

            let short_ident = ident.rsplit('.').next().map(|s| s.to_owned());

            parts.push(Bl3Part {
                ident,
                short_ident,
                idx: part_idx,
            });
        }

        Ok((num_bits, parts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_encrypt_serial() {
        let serial_number: Vec<u8> = vec![
            3, 7, 104, 235, 106, 81, 127, 63, 184, 231, 198, 167, 96, 179, 97, 24, 224, 171, 102,
            232, 245, 72, 182, 213, 98,
        ];

        let unencrypted_base64_serial_number = "BL3(AwAAAABmboC7I9xAEzwShMJVX8nPYwsAAA==)";

        let orig_serial_number = serial_number.clone();

        let decrypted =
            Bl3Item::from_serial_bytes(&serial_number).expect("failed to decrypt serial");

        assert_eq!(decrypted.balance_part.ident, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Balance/InvBalD_Shield_OldGod.InvBalD_Shield_OldGod");
        assert_eq!(
            decrypted.inv_data_part.ident,
            "/Game/Gear/Shields/_Design/A_Data/Shield_Default.Shield_Default"
        );
        assert_eq!(
            decrypted.manufacturer_part.ident,
            "/Game/Gear/Manufacturers/_Design/Hyperion.Hyperion"
        );

        let item_parts = decrypted.item_parts.as_ref().unwrap();

        assert_eq!(item_parts.parts[0].ident, "/Game/Gear/Shields/_Design/PartSets/Part_Manufacturer/Shield_Part_Body_03_Hyperion.Shield_Part_Body_03_Hyperion");
        assert_eq!(item_parts.parts[1].ident, "/Game/Gear/Shields/_Design/PartSets/Part_Rarity/Shield_Part_Rarity_Hyperion_05_Legendary.Shield_Part_Rarity_Hyperion_05_Legendary");
        assert_eq!(item_parts.parts[2].ident, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Parts/Part_Shield_Aug_OldGod.Part_Shield_Aug_OldGod");
        assert_eq!(item_parts.parts[3].ident, "/Game/Gear/Shields/_Design/PartSets/Part_Augment/RechargeRate/Part_Shield_Aug_RechargeRate.Part_Shield_Aug_RechargeRate");
        assert_eq!(item_parts.parts[4].ident, "/Game/Gear/Shields/_Design/PartSets/Part_Augment/Spike/Part_Shield_Aug_Spike.Part_Shield_Aug_Spike");
        assert_eq!(item_parts.parts[5].ident, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Parts/Shield_Part_Element_Fire_OldGod.Shield_Part_Element_Fire_OldGod");
        assert_eq!(item_parts.parts[6].ident, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Parts/Shield_Part_Mat_OldGod.Shield_Part_Mat_OldGod");
        assert_eq!(item_parts.generic_parts[0].ident, "/Game/PatchDLC/Raid1/Gear/Anointed/Generic/SkillEnd_BonusEleDamage_Radiation/GPart_EG_SkillEndBonusEleDamage_Radiation.GPart_EG_SkillEndBonusEleDamage_Radiation");

        let encrypted = decrypted.get_serial_number(true).unwrap();

        assert_eq!(encrypted, orig_serial_number);

        let unencrypted_serial_base64 = decrypted.get_serial_number_base64(false).unwrap();

        assert_eq!(unencrypted_serial_base64, unencrypted_base64_serial_number);

        let decrypted_from_base64 =
            Bl3Item::from_serial_base64(unencrypted_base64_serial_number).unwrap();

        assert_eq!(decrypted.balance_part, decrypted_from_base64.balance_part);
        assert_eq!(
            item_parts.parts,
            decrypted_from_base64.item_parts.as_ref().unwrap().parts
        );
        assert_eq!(
            item_parts.generic_parts,
            decrypted_from_base64
                .item_parts
                .as_ref()
                .unwrap()
                .generic_parts
        );

        let encrypted_serial_base64 = decrypted.get_serial_number_base64(true).unwrap();

        let encrypted_from_base64 = Bl3Item::from_serial_base64(&encrypted_serial_base64).unwrap();

        assert_eq!(decrypted, encrypted_from_base64);
    }

    #[test]
    fn test_decrypt_base64() {
        Bl3Item::from_serial_base64("bl3(BMo1YGLGQ0MGYsI1/FbX0bJzzEAlJV/zmj/7qVR3P7k=)").unwrap();
        Bl3Item::from_serial_base64("bl3(BDcRFWih0RoFBasjJ57Z1Zlf1975cgf2ns3n+pGwL9wo0iSoqfEvpNLcQBqq+kyitN3iuNu36Njp0sLClYQHFp550i9NgKN5J6xn8H2YeH1Ugoqv)").unwrap();
    }
}
