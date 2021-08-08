use std::str::FromStr;

use anyhow::{bail, ensure, Context, Result};
use bitvec::prelude::*;
use byteorder::{BigEndian, WriteBytesExt};
use encoding_rs::mem::decode_latin1;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum::{Display, EnumString};

use crate::bl3_save::arbitrary_bits::ArbitraryBits;
use crate::game_data::{BALANCE_NAME_MAPPING, BALANCE_TO_INV_KEY};
use crate::parser::read_be_signed_int;
use crate::resources::{INVENTORY_PARTS_ALL, INVENTORY_SERIAL_DB};

pub const MAX_BL3_ITEM_PARTS: usize = 63;

// Translated from https://github.com/apocalyptech/bl3-cli-saveedit/blob/master/bl3save/datalib.py
// All credits to apocalyptech

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bl3Item {
    pub serial_version: u8,
    pub orig_seed: i32,
    decrypted_serial: Vec<u8>,
    pub data_version: usize,
    pub balance_part: BalancePart,
    pub part_inv_key: String,
    pub inv_data: String,
    pub inv_data_idx: usize,
    pub inv_data_bits: usize,
    pub manufacturer: String,
    pub manufacturer_short: Option<String>,
    pub manufacturer_idx: usize,
    pub manufacturer_bits: usize,
    pub level: usize,
    pub part_bits: usize,
    pub parts: Vec<Bl3Part>,
    pub generic_part_bits: usize,
    pub generic_parts: Vec<Bl3Part>,
    pub rerolled: usize,
    pub item_type: ItemType,
    pub rarity: ItemRarity,
    pub weapon_type: Option<WeaponType>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct BalancePart {
    pub ident: String,
    pub short_ident: Option<String>,
    pub name: Option<String>,
    pub idx: usize,
    pub bits: usize,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bl3Part {
    pub ident: String,
    pub short_ident: Option<String>,
    pub idx: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Display, EnumString)]
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
}

impl std::default::Default for ItemType {
    fn default() -> Self {
        Self::Weapon
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Display, EnumString)]
pub enum ItemRarity {
    #[strum(serialize = "01/Common", to_string = "Common")]
    Common,
    #[strum(serialize = "02/Uncommon", to_string = "Uncommon")]
    Uncommon,
    #[strum(serialize = "03/Rare", to_string = "Rare")]
    Rare,
    #[strum(serialize = "04/Very Rare", to_string = "Very Rare")]
    VeryRare,
    #[strum(to_string = "Legendary")]
    Legendary,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Display)]
pub enum WeaponType {
    #[strum(to_string = "Pistol")]
    Pistol,
    #[strum(to_string = "Shotgun")]
    Shotgun,
    #[strum(to_string = "SMG")]
    Smg,
    #[strum(to_string = "AR")]
    Ar,
    #[strum(to_string = "Sniper")]
    Sniper,
    #[strum(to_string = "Heavy")]
    Heavy,
}

impl std::default::Default for ItemRarity {
    fn default() -> Self {
        Self::Legendary
    }
}

impl Bl3Item {
    pub fn from_serial_number(serial: Vec<u8>) -> Result<Self> {
        // first decrypt the serial
        let mut serial = serial;

        ensure!(serial.len() >= 5);

        let initial_byte = serial[0];

        if initial_byte != 3 && initial_byte != 4 {
            bail!("serial version was not correct so we will not decrypt this item");
        }

        let serial_version = initial_byte;

        let orig_seed = read_be_signed_int(&serial[1..5])?.1;

        let decrypted_serial = Self::bogodecrypt(&mut serial[5..], orig_seed);

        let orig_checksum = &decrypted_serial[..2];

        let data_to_checksum = [&serial[..5], b"\xFF\xFF", &decrypted_serial[2..]].concat();

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&data_to_checksum);
        let computed_crc = hasher.finalize();

        let mut computed_checksum = Vec::with_capacity(2);

        computed_checksum
            .write_u16::<BigEndian>((((computed_crc >> 16) ^ computed_crc) & 0xFFFF) as u16)?;

        ensure!(orig_checksum == computed_checksum);

        // What we will actually store
        let decrypted_serial = &decrypted_serial[2..];

        // parse the serial data
        let mut bits = ArbitraryBits::new(decrypted_serial.view_bits::<Lsb0>());

        let ident = bits.eat(8)?;

        // Todo: ident can be 0 in some pc saves? check this
        ensure!(ident == 128 || ident == 0);

        let data_version = bits.eat(7)?;

        if data_version > INVENTORY_SERIAL_DB.max_version {
            bail!("cannot parse item as it is newer than the version of this parser")
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

        let item_part_data = &*INVENTORY_PARTS_ALL;
        let item_part_info = balance_short_name
            .as_ref()
            .and_then(|bs| item_part_data.get(bs));

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

        let balance_eng_name = BALANCE_NAME_MAPPING
            .par_iter()
            .find_first(|gd| balance.to_lowercase().contains(gd.ident))
            .map(|gd| gd.name.to_owned());

        let part_inv_key = BALANCE_TO_INV_KEY
            .par_iter()
            .find_first(|gd| balance.to_lowercase().contains(gd.ident))
            .map(|gd| gd.name.to_owned())
            .with_context(|| format!("failed to read part_inv_key: {}", orig_seed))?;

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

        let _additional_data = (0..additional_count)
            .map(|_| bits.eat(8))
            .collect::<Result<Vec<_>>>()?;

        let num_customs = bits.eat(4)?;

        ensure!(num_customs == 0);

        let rerolled = if serial_version >= 4 { bits.eat(8)? } else { 0 };

        if bits.len() > 7 || bits.bitslice().count_ones() > 0 {
            bail!("could not fully parse the weapon data")
        }

        let balance_part = BalancePart {
            ident: balance,
            short_ident: balance_short_name,
            name: balance_eng_name,
            idx: balance_idx,
            bits: balance_bits,
        };

        let item_type = ItemType::from_str(&part_inv_key).unwrap_or_default();

        let decrypted_serial = decrypted_serial.to_vec();

        Ok(Self {
            serial_version,
            orig_seed,
            decrypted_serial,
            data_version,
            balance_part,
            part_inv_key,
            inv_data,
            inv_data_idx,
            inv_data_bits,
            manufacturer,
            manufacturer_short,
            manufacturer_idx,
            manufacturer_bits,
            level,
            part_bits,
            parts,
            generic_part_bits,
            generic_parts,
            rerolled,
            item_type,
            rarity,
            weapon_type,
        })
    }

    pub fn from_serial_base64(serial: &str) -> Result<Self> {
        ensure!(serial.len() > 5);

        let serial_lower = serial.to_lowercase();

        if !serial_lower.starts_with("bl3(") || !serial_lower.ends_with(')') {
            bail!("invalid item serial")
        }

        let decoded = base64::decode(&serial[4..serial.len() - 1])?;

        Self::from_serial_number(decoded)
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

    fn xor_data(data: &mut [u8], seed: i32) {
        if seed != 0 {
            let mut xor = ((seed >> 5) as i64) & 0xFFFFFFFF;

            for d in data.iter_mut() {
                xor = (xor * 0x10A860C1) % 0xFFFFFFFB;
                *d ^= xor as u8;
            }
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
            .get_part(category, part_idx)
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
                .get_part(category, part_idx)
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
            Bl3Item::from_serial_number(serial_number).expect("failed to decrypt serial");

        assert_eq!(decrypted.balance_part.ident, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Balance/InvBalD_Shield_OldGod.InvBalD_Shield_OldGod");
        assert_eq!(
            decrypted.inv_data,
            "/Game/Gear/Shields/_Design/A_Data/Shield_Default.Shield_Default"
        );
        assert_eq!(
            decrypted.manufacturer,
            "/Game/Gear/Manufacturers/_Design/Hyperion.Hyperion"
        );
        assert_eq!(decrypted.parts[0].ident, "/Game/Gear/Shields/_Design/PartSets/Part_Manufacturer/Shield_Part_Body_03_Hyperion.Shield_Part_Body_03_Hyperion");
        assert_eq!(decrypted.parts[1].ident, "/Game/Gear/Shields/_Design/PartSets/Part_Rarity/Shield_Part_Rarity_Hyperion_05_Legendary.Shield_Part_Rarity_Hyperion_05_Legendary");
        assert_eq!(decrypted.parts[2].ident, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Parts/Part_Shield_Aug_OldGod.Part_Shield_Aug_OldGod");
        assert_eq!(decrypted.parts[3].ident, "/Game/Gear/Shields/_Design/PartSets/Part_Augment/RechargeRate/Part_Shield_Aug_RechargeRate.Part_Shield_Aug_RechargeRate");
        assert_eq!(decrypted.parts[4].ident, "/Game/Gear/Shields/_Design/PartSets/Part_Augment/Spike/Part_Shield_Aug_Spike.Part_Shield_Aug_Spike");
        assert_eq!(decrypted.parts[5].ident, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Parts/Shield_Part_Element_Fire_OldGod.Shield_Part_Element_Fire_OldGod");
        assert_eq!(decrypted.parts[6].ident, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Parts/Shield_Part_Mat_OldGod.Shield_Part_Mat_OldGod");
        assert_eq!(decrypted.generic_parts[0].ident, "/Game/PatchDLC/Raid1/Gear/Anointed/Generic/SkillEnd_BonusEleDamage_Radiation/GPart_EG_SkillEndBonusEleDamage_Radiation.GPart_EG_SkillEndBonusEleDamage_Radiation");

        let encrypted = decrypted.get_serial_number(true).unwrap();

        assert_eq!(encrypted, orig_serial_number);

        let unencrypted_serial_base64 = decrypted.get_serial_number_base64(false).unwrap();

        assert_eq!(unencrypted_serial_base64, unencrypted_base64_serial_number);

        let decrypted_from_base64 =
            Bl3Item::from_serial_base64(unencrypted_base64_serial_number).unwrap();

        assert_eq!(decrypted.balance_part, decrypted_from_base64.balance_part);
        assert_eq!(decrypted.parts, decrypted_from_base64.parts);
        assert_eq!(decrypted.generic_parts, decrypted_from_base64.generic_parts);

        let encrypted_serial_base64 = decrypted.get_serial_number_base64(true).unwrap();

        let encrypted_from_base64 = Bl3Item::from_serial_base64(&encrypted_serial_base64).unwrap();

        assert_eq!(decrypted, encrypted_from_base64);
    }
}
