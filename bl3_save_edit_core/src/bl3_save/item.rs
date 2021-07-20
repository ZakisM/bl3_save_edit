use anyhow::{bail, Context, Result};
use byteorder::{BigEndian, WriteBytesExt};

use crate::parser::read_be_int;

pub struct Item;

impl Item {
    pub fn from_serial_number(mut serial_number: Vec<u8>) {
        decrypt_serial(&mut serial_number);
    }
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
    xor_data(data, seed);

    let steps = (seed & 0x1F) as usize % (data.len());

    let first_half = &data[steps..];
    let second_half = &data[..steps];

    [first_half, second_half].concat()
}

fn decrypt_serial(serial: &mut [u8]) -> Result<()> {
    if serial.len() < 5 {
        bail!("serial length was too short to read")
    }

    let initial_byte = serial[0];

    if initial_byte != 3 && initial_byte != 4 {
        bail!("initial byte was not correct so we will not decrypt this item");
    }

    let serial_version = initial_byte;

    let orig_seed = read_be_int(&serial[1..5])?.1;

    println!("{}", orig_seed);

    let decrypted = bogodecrypt(&mut serial[5..], orig_seed);

    println!("{:?}", decrypted);

    let orig_checksum = &decrypted[..2];

    println!("{:?}", orig_checksum);

    let data_to_checksum = [&serial[..5], b"\xFF\xFF", &decrypted[2..]].concat();
    println!("{:?}", &data_to_checksum);

    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&data_to_checksum);
    let computed_crc = hasher.finalize();
    println!("{}", computed_crc);

    let mut computed_checksum = Vec::with_capacity(2);

    computed_checksum
        .write_u16::<BigEndian>((((computed_crc >> 16) ^ computed_crc) & 0xFFFF) as u16)?;

    println!("{:?}", computed_checksum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_serial() {
        let mut serial_number = vec![
            3, 7, 104, 235, 106, 81, 127, 63, 184, 231, 198, 167, 96, 179, 97, 24, 224, 171, 102,
            232, 245, 72, 182, 213, 98,
        ];

        // let expected_decrypted = [5, 168, 128, 187, 35, 220, 64, 19, 60, 18, 132, 194, 85, 95, 201, 207, 99, 11, 0, 0];

        let decrypted = decrypt_serial(&mut serial_number).expect("failed to decrypt serial");

        // assert_eq!(decrypted, expected_decrypted);
    }
}
