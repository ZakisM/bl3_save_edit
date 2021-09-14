use anyhow::{bail, Result};
use bitvec::prelude::*;

// Inspired from https://github.com/apocalyptech/bl3-cli-saveedit/blob/master/bl3save/datalib.py
// Thanks apocalyptech

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
            bail!("Invalid range passed to bit slice.")
        } else {
            let res = self.bitslice[0..num_bits].load_le::<usize>();
            self.bitslice = &self.bitslice[num_bits..];
            Ok(res)
        }
    }
}

#[derive(Debug, Default)]
pub struct ArbitraryBitVec<O = Lsb0, T = usize>
where
    O: BitOrder,
    T: BitStore,
{
    pub bitvec: BitVec<O, T>,
    pub curr_index: usize,
}

impl<O, T> ArbitraryBitVec<O, T>
where
    O: BitOrder,
    T: BitStore,
    BitSlice<O, T>: BitField,
{
    pub fn new() -> Self {
        ArbitraryBitVec {
            bitvec: BitVec::<O, T>::new(),
            curr_index: 0,
        }
    }

    pub fn append_le(&mut self, value: usize, num_bits: usize) {
        self.bitvec.resize(self.bitvec.len() + num_bits, false);

        let index = self.curr_index..num_bits + self.curr_index;

        // fit the value into the smallest bit sized integer
        match num_bits {
            b if b <= 8 => {
                self.bitvec[index].store_le::<u8>(value as u8);
            }
            b if b <= 16 => {
                self.bitvec[index].store_le::<u16>(value as u16);
            }
            b if b <= 32 => {
                self.bitvec[index].store_le::<u32>(value as u32);
            }
            _ => {
                self.bitvec[index].store_le::<usize>(value);
            }
        }

        self.curr_index += num_bits;
    }
}
