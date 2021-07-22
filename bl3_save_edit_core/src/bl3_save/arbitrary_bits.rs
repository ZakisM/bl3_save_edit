use anyhow::{bail, Result};
use bitvec::prelude::*;

// Translated from https://github.com/apocalyptech/bl3-cli-saveedit/blob/master/bl3save/datalib.py
// All credits to apocalyptech

#[derive(Debug)]
pub struct ArbitraryBits<'a> {
    // We are using the bitvec library here instead of parsing with nom as the bit lengths do not
    // always fit into chunks of 8 and nom will drop any bits that do not fit into chunks of 8
    bitslice: &'a BitSlice<Lsb0, u8>,
}

impl<'a> ArbitraryBits<'a> {
    pub fn new(bitslice: &'a BitSlice<Lsb0, u8>) -> Self {
        ArbitraryBits { bitslice }
    }

    pub fn bitslice(&self) -> &'a BitSlice<Lsb0, u8> {
        self.bitslice
    }

    pub fn is_empty(&self) -> bool {
        self.bitslice.is_empty()
    }

    pub fn len(&self) -> usize {
        self.bitslice.len()
    }

    pub fn eat(&mut self, num_bits: usize) -> Result<usize> {
        if num_bits > self.bitslice.len() {
            bail!("invalid range passed to bit slice")
        } else {
            let res = self.bitslice[0..num_bits].load_le::<usize>();
            self.bitslice = &self.bitslice[num_bits..];
            Ok(res)
        }
    }
}
