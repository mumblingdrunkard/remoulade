// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright (C) 2024 mumblingdrunkard

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
