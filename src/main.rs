use std::fs;
use std::io::Read;

use anyhow::Result;

use crate::bl3_save::Bl3Save;

mod bl3_save;
mod error;
mod parser;
mod protos;

fn main() -> Result<()> {
    let mut save_file = fs::File::open("./test_files/1.sav")?;
    let mut save_file_data = Vec::with_capacity(save_file.metadata()?.len() as usize);

    save_file.read_to_end(&mut save_file_data)?;

    let bl3_save = Bl3Save::from_data(&mut save_file_data)?;

    println!("{}", bl3_save);

    Ok(())
}
