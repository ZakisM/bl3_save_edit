use anyhow::{bail, ensure, Context, Result};
use bitvec::prelude::*;
use byteorder::{BigEndian, WriteBytesExt};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::bl3_save::arbitrary_bits::ArbitraryBits;
use crate::game_data::{BALANCE_NAME_MAPPING, BALANCE_TO_INV_KEY};
use crate::parser::read_be_signed_int;
use crate::resources::INVENTORY_SERIAL_DB;

pub const MAX_PARTS: usize = 63;

// Translated from https://github.com/apocalyptech/bl3-cli-saveedit/blob/master/bl3save/datalib.py
// All credits to apocalyptech

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bl3Serial {
    pub header_version: u8,
    pub data_version: usize,
    pub balance_part: ItemPart,
    pub part_inv_key: String,
    pub inv_data: String,
    pub inv_data_idx: usize,
    pub inv_data_bits: usize,
    pub manufacturer: String,
    pub manufacturer_idx: usize,
    pub manufacturer_bits: usize,
    pub level: usize,
    pub part_bits: usize,
    pub parts: Vec<(String, usize)>,
    pub generic_part_bits: usize,
    pub generic_parts: Vec<(String, usize)>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct ItemPart {
    pub ident: String,
    pub short_name: String,
    pub name: Option<String>,
    pub idx: usize,
    pub bits: usize,
}

impl Bl3Serial {
    pub fn from_serial_number(serial: Vec<u8>) -> Result<Self> {
        // first decrypt the serial
        let mut serial = serial;

        ensure!(serial.len() >= 5);

        let initial_byte = serial[0];

        if initial_byte != 3 && initial_byte != 4 {
            bail!("serial version was not correct so we will not decrypt this item");
        }

        let header_version = initial_byte;

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

        // parse the serial data
        let mut bits = ArbitraryBits::new(decrypted_serial[2..].view_bits::<Lsb0>());

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

        let level = bits.eat(7)?;

        let balance_short_name = balance
            .rsplit('.')
            .next()
            .map(|s| s.to_owned())
            .with_context(|| format!("failed to read balance_short_name for: {}", balance))?;

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

        let additional_data = (0..additional_count)
            .map(|_| bits.eat(8))
            .collect::<Result<Vec<_>>>()?;

        let num_customs = bits.eat(4)?;

        let rerolled = if header_version >= 4 { bits.eat(8)? } else { 0 };

        if bits.len() > 7 || bits.bitslice().count_ones() > 0 {
            bail!("could not fully parse the weapon data")
        }

        let balance_part = ItemPart {
            ident: balance,
            short_name: balance_short_name,
            name: balance_eng_name,
            idx: balance_idx,
            bits: balance_bits,
        };

        Ok(Self {
            header_version,
            data_version,
            balance_part,
            part_inv_key,
            inv_data,
            inv_data_idx,
            inv_data_bits,
            manufacturer,
            manufacturer_idx,
            manufacturer_bits,
            level,
            part_bits,
            parts,
            generic_part_bits,
            generic_parts,
        })
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
    ) -> Result<(usize, Vec<(String, usize)>)> {
        let num_bits = INVENTORY_SERIAL_DB.get_num_bits(category, version)?;
        let num_parts = bits.eat(count_bits)?;

        let mut parts = Vec::with_capacity(num_parts);

        for _ in 0..num_parts {
            let part_idx = bits.eat(num_bits)?;

            let part_val = INVENTORY_SERIAL_DB
                .get_part(category, part_idx)
                .unwrap_or_else(|_| "unknown".to_owned());

            parts.push((part_val, part_idx));
        }

        Ok((num_bits, parts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_serial() {
        let serial_number: Vec<u8> = vec![
            3, 7, 104, 235, 106, 81, 127, 63, 184, 231, 198, 167, 96, 179, 97, 24, 224, 171, 102,
            232, 245, 72, 182, 213, 98,
        ];

        let decrypted =
            Bl3Serial::from_serial_number(serial_number).expect("failed to decrypt serial");

        assert_eq!(decrypted.balance_part.ident, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Balance/InvBalD_Shield_OldGod.InvBalD_Shield_OldGod");
        assert_eq!(
            decrypted.inv_data,
            "/Game/Gear/Shields/_Design/A_Data/Shield_Default.Shield_Default"
        );
        assert_eq!(
            decrypted.manufacturer,
            "/Game/Gear/Manufacturers/_Design/Hyperion.Hyperion"
        );
        assert_eq!(decrypted.parts[0].0, "/Game/Gear/Shields/_Design/PartSets/Part_Manufacturer/Shield_Part_Body_03_Hyperion.Shield_Part_Body_03_Hyperion");
        assert_eq!(decrypted.parts[1].0, "/Game/Gear/Shields/_Design/PartSets/Part_Rarity/Shield_Part_Rarity_Hyperion_05_Legendary.Shield_Part_Rarity_Hyperion_05_Legendary");
        assert_eq!(decrypted.parts[2].0, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Parts/Part_Shield_Aug_OldGod.Part_Shield_Aug_OldGod");
        assert_eq!(decrypted.parts[3].0, "/Game/Gear/Shields/_Design/PartSets/Part_Augment/RechargeRate/Part_Shield_Aug_RechargeRate.Part_Shield_Aug_RechargeRate");
        assert_eq!(decrypted.parts[4].0, "/Game/Gear/Shields/_Design/PartSets/Part_Augment/Spike/Part_Shield_Aug_Spike.Part_Shield_Aug_Spike");
        assert_eq!(decrypted.parts[5].0, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Parts/Shield_Part_Element_Fire_OldGod.Shield_Part_Element_Fire_OldGod");
        assert_eq!(decrypted.parts[6].0, "/Game/PatchDLC/Hibiscus/Gear/Shields/_Unique/OldGod/Parts/Shield_Part_Mat_OldGod.Shield_Part_Mat_OldGod");
        assert_eq!(decrypted.generic_parts[0].0, "/Game/PatchDLC/Raid1/Gear/Anointed/Generic/SkillEnd_BonusEleDamage_Radiation/GPart_EG_SkillEndBonusEleDamage_Radiation.GPart_EG_SkillEndBonusEleDamage_Radiation");
    }
}
