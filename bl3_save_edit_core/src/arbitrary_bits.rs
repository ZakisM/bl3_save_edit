use anyhow::{bail, Result};
use bitvec::field::BitField;
use bitvec::prelude::*;

// Inspired from https://github.com/apocalyptech/bl3-cli-saveedit/blob/master/bl3save/datalib.py
// Thanks apocalyptech

#[derive(Debug)]
pub struct ArbitraryBits<'a> {
    // We are using the bitvec library here instead of parsing with nom as the bit lengths do not
    // always fit into chunks of 8 and nom will drop any bits that do not fit into chunks of 8
    bitslice: &'a BitSlice<u8, Lsb0>,
}

impl<'a> ArbitraryBits<'a> {
    pub fn new(bitslice: &'a BitSlice<u8, Lsb0>) -> Self {
        ArbitraryBits { bitslice }
    }

    pub fn bitslice(&self) -> &'a BitSlice<u8, Lsb0> {
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
            let (head, tail) = self.bitslice.split_at(num_bits);
            let res = head.load_le::<usize>();
            self.bitslice = tail;
            Ok(res)
        }
    }
}

#[derive(Debug, Default)]
pub struct ArbitraryBitVec<T = usize, O = Lsb0>
where
    T: BitStore,
    O: BitOrder,
{
    pub bitvec: BitVec<T, O>,
    pub curr_index: usize,
}

impl<T, O> ArbitraryBitVec<T, O>
where
    T: BitStore,
    O: BitOrder,
    BitSlice<T, O>: BitField,
{
    pub fn new() -> Self {
        ArbitraryBitVec {
            bitvec: BitVec::<T, O>::new(),
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
