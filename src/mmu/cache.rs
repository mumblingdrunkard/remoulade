use std::marker::PhantomData;

use crate::inst::Instruction;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum FourWay {
    A = 0,
    B,
    C,
    D,
}

impl FourWay {
    fn from_u32(n: u32) -> Option<Self> {
        use FourWay::*;
        let way = match n {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            _ => None?,
        };
        Some(way)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
enum FourWayLru {
    ABCD,
    ABDC,
    ACBD,
    ACDB,
    ADBC,
    ADCB,
    BACD,
    BADC,
    BCAD,
    BCDA,
    BDAC,
    BDCA,
    CABD,
    CADB,
    CBAD,
    CBDA,
    CDAB,
    CDBA,
    DABC,
    DACB,
    DBAC,
    DBCA,
    DCAB,
    DCBA,
}

impl FourWayLru {
    fn touch(&mut self, way: FourWay) {
        use FourWay::*;
        use FourWayLru::*;
        *self = match (way, *self) {
            (A, BACD) => ABCD,
            (A, BADC) => ABDC,
            (A, BCAD) => ABCD,
            (A, BCDA) => ABCD,
            (A, BDAC) => ABDC,
            (A, BDCA) => ABDC,
            (A, CABD) => ACBD,
            (A, CADB) => ACDB,
            (A, CBAD) => ACBD,
            (A, CBDA) => ACBD,
            (A, CDAB) => ACDB,
            (A, CDBA) => ACDB,
            (A, DABC) => ADBC,
            (A, DACB) => ADCB,
            (A, DBAC) => ADBC,
            (A, DBCA) => ADBC,
            (A, DCAB) => ADCB,
            (A, DCBA) => ADCB,

            (B, ABCD) => BACD,
            (B, ABDC) => BADC,
            (B, ACBD) => BACD,
            (B, ACDB) => BACD,
            (B, ADBC) => BADC,
            (B, ADCB) => BADC,
            (B, CABD) => BCAD,
            (B, CADB) => BCAD,
            (B, CBAD) => BCAD,
            (B, CBDA) => BCDA,
            (B, CDAB) => BCDA,
            (B, CDBA) => BCDA,
            (B, DABC) => BDAC,
            (B, DACB) => BDAC,
            (B, DBAC) => BDAC,
            (B, DBCA) => BDCA,
            (B, DCAB) => BDCA,
            (B, DCBA) => BDCA,

            (C, ABCD) => CABD,
            (C, ABDC) => CABD,
            (C, ACBD) => CABD,
            (C, ACDB) => CADB,
            (C, ADBC) => CADB,
            (C, ADCB) => CADB,
            (C, BACD) => CBAD,
            (C, BADC) => CBAD,
            (C, BCAD) => CBAD,
            (C, BCDA) => CBDA,
            (C, BDAC) => CBDA,
            (C, BDCA) => CBDA,
            (C, DABC) => CDAB,
            (C, DACB) => CDAB,
            (C, DBAC) => CDBA,
            (C, DBCA) => CDBA,
            (C, DCAB) => CDAB,
            (C, DCBA) => CDBA,

            (D, ABCD) => DABC,
            (D, ABDC) => DABC,
            (D, ACBD) => DACB,
            (D, ACDB) => DACB,
            (D, ADBC) => DABC,
            (D, ADCB) => DACB,
            (D, BACD) => DBAC,
            (D, BADC) => DBAC,
            (D, BCAD) => DBCA,
            (D, BCDA) => DBCA,
            (D, BDAC) => DBAC,
            (D, BDCA) => DBCA,
            (D, CABD) => DCAB,
            (D, CADB) => DCAB,
            (D, CBAD) => DCBA,
            (D, CBDA) => DCBA,
            (D, CDAB) => DCAB,
            (D, CDBA) => DCBA,

            _ => *self,
        };
    }

    fn replace(&mut self) -> FourWay {
        use FourWay::*;
        use FourWayLru::*;
        let way = match *self {
            ABCD => A,
            ABDC => A,
            ACBD => A,
            ACDB => A,
            ADBC => A,
            ADCB => A,

            BACD => B,
            BADC => B,
            BCAD => B,
            BCDA => B,
            BDAC => B,
            BDCA => B,

            CABD => C,
            CADB => C,
            CBAD => C,
            CBDA => C,
            CDAB => C,
            CDBA => C,

            DABC => D,
            DACB => D,
            DBAC => D,
            DBCA => D,
            DCAB => D,
            DCBA => D,
        };
        self.touch(way);
        way
    }
}

#[derive(Debug)]
pub struct RawIndex<T> {
    set: u8,
    way: FourWay,
    _pd: PhantomData<T>,
}

impl<T> Clone for RawIndex<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for RawIndex<T> {}

struct InstructionCacheParameters;

impl InstructionCacheParameters {
    pub const SETS: usize = 256;
    pub const ASSOCIATIVITY: usize = 4;
    pub const BLOCK_SIZE: usize = 16;
}

type InstructionCacheSetValidList = [bool; InstructionCacheParameters::ASSOCIATIVITY];
type InstructionCacheValidList = [InstructionCacheSetValidList; InstructionCacheParameters::SETS];
type InstructionCacheSetTagList = [u32; InstructionCacheParameters::ASSOCIATIVITY];
type InstructionCacheTagList = [InstructionCacheSetTagList; InstructionCacheParameters::SETS];
type InstructionCacheLruInfo = [FourWayLru; InstructionCacheParameters::SETS];
type InstructionCacheBlock = [Instruction; InstructionCacheParameters::BLOCK_SIZE];
type InstructionCacheSet = [InstructionCacheBlock; InstructionCacheParameters::ASSOCIATIVITY];
type InstructionCacheData = [InstructionCacheSet; InstructionCacheParameters::SETS];

/// A read-only instruction cache
pub struct InstructionCache<const ID: usize> {
    lut: [Option<(u32, RawIndex<Self>)>; 2],
    data: InstructionCacheData,
    tags: InstructionCacheTagList,
    valid: InstructionCacheValidList,
    lru_info: InstructionCacheLruInfo,
}

impl<const ID: usize> InstructionCache<ID> {
    pub fn new() -> Self {
        Self {
            lut: [None; 2],
            data: [[[Instruction::default(); InstructionCacheParameters::BLOCK_SIZE];
                InstructionCacheParameters::ASSOCIATIVITY];
                InstructionCacheParameters::SETS],
            tags: [[0; InstructionCacheParameters::ASSOCIATIVITY];
                InstructionCacheParameters::SETS],
            valid: [[false; InstructionCacheParameters::ASSOCIATIVITY];
                InstructionCacheParameters::SETS],
            lru_info: [FourWayLru::ABCD; InstructionCacheParameters::SETS],
        }
    }

    fn has_block_number(&self, number: u32) -> Option<RawIndex<Self>> {
        let set = number as usize & 0xff;
        let tag = number >> 8;
        for way in 0..InstructionCacheParameters::ASSOCIATIVITY {
            if self.tags[set][way] == tag && self.valid[set][way] {
                return Some(RawIndex {
                    set: set as u8,
                    way: FourWay::from_u32(way as u32).unwrap(),
                    _pd: PhantomData,
                });
            }
        }
        None
    }

    fn touch_raw_index(&mut self, idx: RawIndex<Self>) {
        self.lru_info[idx.set as usize].touch(idx.way);
    }

    fn get_block_by_raw_index(&mut self, idx: RawIndex<Self>) -> &InstructionCacheBlock {
        let (set, way) = (idx.set as usize, idx.way as usize);
        self.lru_info[set].touch(idx.way);
        &self.data[set][way]
    }

    pub fn read(&mut self, address: u32) -> Option<Instruction> {
        let number = address >> 6;
        let offset = (address as usize >> 2) & 0xf;
        if let Some((n, idx)) = self.lut[0] {
            if n == number {
                return Some(self.get_block_by_raw_index(idx)[offset]);
            }
        }

        if let Some((n, idx)) = self.lut[1] {
            if n == number {
                return Some(self.get_block_by_raw_index(idx)[offset]);
            }
        }

        let idx = self.has_block_number(number)?;
        Some(self.get_block_by_raw_index(idx)[offset])
    }

    pub fn insert(&mut self, number: u32, block: InstructionCacheBlock) -> RawIndex<Self> {
        if let Some(idx) = self.has_block_number(number) {
            self.touch_raw_index(idx);
            return idx;
        }

        let set = number as usize & 0xff;
        let tag = number >> 8;
        let way = self.lru_info[set].replace();

        self.data[set][way as usize] = block;
        self.tags[set][way as usize] = tag;

        RawIndex {
            set: set as u8,
            way,
            _pd: PhantomData,
        }
    }
}

impl<const ID: usize> Default for InstructionCache<ID> {
    fn default() -> Self {
        Self::new()
    }
}

struct DataCacheParameters;

impl DataCacheParameters {
    pub const SETS: usize = 256;
    pub const ASSOCIATIVITY: usize = 4;
    pub const BLOCK_SIZE: usize = 64;
}

type DataCacheSetValidList = [bool; DataCacheParameters::ASSOCIATIVITY];
type DataCacheValidList = [DataCacheSetValidList; DataCacheParameters::SETS];
type DataCacheSetTagList = [u32; DataCacheParameters::ASSOCIATIVITY];
type DataCacheTagList = [DataCacheSetTagList; DataCacheParameters::SETS];
type DataCacheLruInfo = [FourWayLru; DataCacheParameters::SETS];
type DataCacheBlock = [u8; DataCacheParameters::BLOCK_SIZE];
type DataCacheSet = [DataCacheBlock; DataCacheParameters::ASSOCIATIVITY];
type DataCacheData = [DataCacheSet; DataCacheParameters::SETS];

/// A read-only instruction cache
pub struct DataCache<const ID: usize> {
    data: DataCacheData,
    tags: DataCacheTagList,
    valid: DataCacheValidList,
    lru_info: DataCacheLruInfo,
}

impl<const ID: usize> DataCache<ID> {
    pub fn new() -> Self {
        Self {
            data: [[[0; DataCacheParameters::BLOCK_SIZE]; DataCacheParameters::ASSOCIATIVITY];
                DataCacheParameters::SETS],
            tags: [[0; DataCacheParameters::ASSOCIATIVITY]; DataCacheParameters::SETS],
            valid: [[false; DataCacheParameters::ASSOCIATIVITY]; DataCacheParameters::SETS],
            lru_info: [FourWayLru::ABCD; DataCacheParameters::SETS],
        }
    }

    fn has_block_number(&self, number: u32) -> Option<RawIndex<Self>> {
        let set = number as usize & 0xff;
        let tag = number >> 8;
        for way in 0..DataCacheParameters::ASSOCIATIVITY {
            if self.tags[set][way] == tag && self.valid[set][way] {
                return Some(RawIndex {
                    set: set as u8,
                    way: FourWay::from_u32(way as u32).unwrap(),
                    _pd: PhantomData,
                });
            }
        }
        None
    }

    fn touch_raw_index(&mut self, idx: RawIndex<Self>) {
        self.lru_info[idx.set as usize].touch(idx.way);
    }

    pub fn get_block_by_number(&mut self, number: u32) -> Option<&DataCacheBlock> {
        let idx = self.has_block_number(number)?;
        Some(self.get_block_by_raw_index(idx))
    }

    pub fn get_block_by_raw_index(&mut self, idx: RawIndex<Self>) -> &DataCacheBlock {
        let (set, way) = (idx.set as usize, idx.way as usize);
        self.lru_info[set].touch(idx.way);
        &self.data[set][way]
    }

    pub fn write_byte_with_raw_index(&mut self, idx: RawIndex<Self>, offset: usize, byte: u8) {
        todo!()
    }

    pub fn write_half_word_with_raw_index(
        &mut self,
        idx: RawIndex<Self>,
        offset: usize,
        half_word: u16,
    ) {
        todo!()
    }

    pub fn write_word_with_raw_index(&mut self, idx: RawIndex<Self>, offset: usize, word: u32) {
        todo!()
    }

    pub fn insert_block(&mut self, number: u32, block: DataCacheBlock) -> RawIndex<Self> {
        if let Some(idx) = self.has_block_number(number) {
            self.touch_raw_index(idx);
            return idx;
        }

        let set = number as usize & 0xff;
        let tag = number >> 8;
        let way = self.lru_info[set].replace();

        self.data[set][way as usize] = block;
        self.tags[set][way as usize] = tag;

        RawIndex {
            set: set as u8,
            way,
            _pd: PhantomData,
        }
    }
}

impl<const ID: usize> Default for DataCache<ID> {
    fn default() -> Self {
        Self::new()
    }
}
