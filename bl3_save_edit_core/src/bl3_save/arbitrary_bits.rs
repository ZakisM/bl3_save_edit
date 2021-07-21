use anyhow::{bail, Result};
use bitvec::prelude::*;

pub struct ArbitraryBits<'a> {
    curr_read_i: usize,
    // We are using the bitvec library here instead of parsing with nom as the bit lengths do not
    // always fit into chunks of 8 and nom will drop any bits that do not fit into chunks of 8
    bitslice: &'a BitSlice<Lsb0, u8>,
}

impl<'a> ArbitraryBits<'a> {
    pub fn new(bitslice: &'a BitSlice<Lsb0, u8>) -> Self {
        ArbitraryBits {
            curr_read_i: 0,
            bitslice,
        }
    }

    pub fn data(&mut self, num_bits: usize) -> Result<usize> {
        if self.curr_read_i > self.bitslice.len() || num_bits > self.bitslice.len() {
            bail!("invalid range passed to slice")
        } else {
            let start = self.curr_read_i;
            self.curr_read_i += num_bits;
            Ok(self.bitslice[start..num_bits + start].load::<usize>())
        }
    }
}
