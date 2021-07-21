use anyhow::{bail, ensure, Context, Result};
use bitvec::prelude::*;
use byteorder::{BigEndian, WriteBytesExt};

use crate::bl3_save::arbitrary_bits::ArbitraryBits;
use crate::error::BL3ParserError;
use crate::models::inventory_serial_db::InventorySerialDb;
use crate::parser::read_be_int;

pub struct Bl3Serial;

impl Bl3Serial {
    pub fn from_serial_number(serial_number: Vec<u8>) -> Result<()> {
        Self::decrypt_serial(serial_number)?;

        Ok(())
    }

    fn xor_data(data: &mut [u8], seed: u32) {
        if seed != 0 {
            let mut xor = (seed >> 5) as u64;

            for d in data.iter_mut() {
                xor = (xor * 0x10A860C1) % 0xFFFFFFFB;
                *d ^= xor as u8;
            }
        }
    }

    fn bogodecrypt(data: &mut [u8], seed: u32) -> Vec<u8> {
        Self::xor_data(data, seed);

        let steps = (seed & 0x1F) as usize % (data.len());

        let first_half = &data[steps..];
        let second_half = &data[..steps];

        [first_half, second_half].concat()
    }

    fn decrypt_serial(serial: Vec<u8>) -> Result<()> {
        let mut serial = serial;

        ensure!(serial.len() >= 5);

        let initial_byte = serial[0];

        if initial_byte != 3 && initial_byte != 4 {
            bail!("initial byte was not correct so we will not decrypt this item");
        }

        let serial_version = initial_byte;

        let orig_seed = read_be_int(&serial[1..5])?.1;

        // println!("{}", orig_seed);

        let decrypted = Self::bogodecrypt(&mut serial[5..], orig_seed);

        // println!("{:?}", decrypted);

        let orig_checksum = &decrypted[..2];

        // println!("{:?}", orig_checksum);

        let data_to_checksum = [&serial[..5], b"\xFF\xFF", &decrypted[2..]].concat();
        // println!("{:?}", &data_to_checksum);

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&data_to_checksum);
        let computed_crc = hasher.finalize();
        // println!("{}", computed_crc);

        let mut computed_checksum = Vec::with_capacity(2);

        computed_checksum
            .write_u16::<BigEndian>((((computed_crc >> 16) ^ computed_crc) & 0xFFFF) as u16)?;

        // println!("{:?}", computed_checksum);

        ensure!(orig_checksum == computed_checksum);

        Self::parse_serial(&decrypted[2..])?;

        Ok(())
    }

    fn parse_serial(decrypted_serial: &[u8]) -> Result<()> {
        println!("{:?}", decrypted_serial);

        let mut bits = ArbitraryBits::new(decrypted_serial.view_bits::<Lsb0>());

        let ident = bits.data(8)?;

        ensure!(ident == 128);

        let serial_version = bits.data(7)?;

        let inventory_serial_db = InventorySerialDb::load()?;

        let balance = Self::inv_db_header_part(
            &inventory_serial_db,
            "InventoryBalanceData",
            &mut bits,
            serial_version,
        )?;

        dbg!(&balance);

        let inv_data = Self::inv_db_header_part(
            &inventory_serial_db,
            "InventoryData",
            &mut bits,
            serial_version,
        )?;

        dbg!(&inv_data);

        let manufacturer = Self::inv_db_header_part(
            &inventory_serial_db,
            "ManufacturerData",
            &mut bits,
            serial_version,
        )?;

        dbg!(&manufacturer);

        Ok(())
    }

    fn inv_db_header_part(
        inventory_serial_db: &InventorySerialDb,
        category: &str,
        bits: &mut ArbitraryBits,
        serial_version: usize,
    ) -> Result<String> {
        let mut num_bits = inventory_serial_db.get_num_bits(category, serial_version)?;

        let part_idx = bits.data(num_bits)?;

        let part = inventory_serial_db
            .get_part(category, part_idx)
            .unwrap_or_else(|_| "unknown".to_owned());

        Ok(part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_serial() {
        let mut serial_number: Vec<u8> = vec![
            3, 7, 104, 235, 106, 81, 127, 63, 184, 231, 198, 167, 96, 179, 97, 24, 224, 171, 102,
            232, 245, 72, 182, 213, 98,
        ];

        // let expected_decrypted = [5, 168, 128, 187, 35, 220, 64, 19, 60, 18, 132, 194, 85, 95, 201, 207, 99, 11, 0, 0];

        let decrypted =
            Bl3Serial::from_serial_number(serial_number).expect("failed to decrypt serial");

        // assert_eq!(decrypted, expected_decrypted);
    }
}
