// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright (C) 2024 mumblingdrunkard

//! Module containing register file and register types, as well as functions and utilities to use
//! them effectively.

pub trait RegType {}
impl RegType for u32 {}
impl RegType for u64 {}
impl RegType for u128 {}

pub struct RegFile<T: RegType> {
    reg: [T; 33],
}

impl<T: Default + RegType + Copy> Default for RegFile<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default + RegType + Copy> RegFile<T> {
    pub fn new() -> Self {
        Self {
            reg: [T::default(); 33],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRs1 {
    X0 = 0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRs2 {
    X0 = 0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRd {
    X0 = 32,
    X1 = 1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

impl IRs1 {
    pub fn checked_from_u32(b: u32) -> Option<Self> {
        let res = match b {
            0..=31 => Self::wrapping_from_u32(b),
            _ => None?,
        };

        Some(res)
    }

    pub fn wrapping_from_u32(b: u32) -> Self {
        match b & 0x1f {
            0 => IRs1::X0,
            1 => IRs1::X1,
            2 => IRs1::X2,
            3 => IRs1::X3,
            4 => IRs1::X4,
            5 => IRs1::X5,
            6 => IRs1::X6,
            7 => IRs1::X7,
            8 => IRs1::X8,
            9 => IRs1::X9,
            10 => IRs1::X10,
            11 => IRs1::X11,
            12 => IRs1::X12,
            13 => IRs1::X13,
            14 => IRs1::X14,
            15 => IRs1::X15,
            16 => IRs1::X16,
            17 => IRs1::X17,
            18 => IRs1::X18,
            19 => IRs1::X19,
            20 => IRs1::X20,
            21 => IRs1::X21,
            22 => IRs1::X22,
            23 => IRs1::X23,
            24 => IRs1::X24,
            25 => IRs1::X25,
            26 => IRs1::X26,
            27 => IRs1::X27,
            28 => IRs1::X28,
            29 => IRs1::X29,
            30 => IRs1::X30,
            31 => IRs1::X31,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        Self::checked_from_u32(raw32 >> 15 & 0x1f).expect("Extracted value was larger than 5 bits")
    }
}

impl IRs2 {
    pub fn checked_from_u32(b: u32) -> Option<Self> {
        let res = match b {
            0..=31 => Self::wrapping_from_u32(b),
            _ => None?,
        };
        Some(res)
    }

    pub fn wrapping_from_u32(b: u32) -> Self {
        match b & 0x1f {
            0 => IRs2::X0,
            1 => IRs2::X1,
            2 => IRs2::X2,
            3 => IRs2::X3,
            4 => IRs2::X4,
            5 => IRs2::X5,
            6 => IRs2::X6,
            7 => IRs2::X7,
            8 => IRs2::X8,
            9 => IRs2::X9,
            10 => IRs2::X10,
            11 => IRs2::X11,
            12 => IRs2::X12,
            13 => IRs2::X13,
            14 => IRs2::X14,
            15 => IRs2::X15,
            16 => IRs2::X16,
            17 => IRs2::X17,
            18 => IRs2::X18,
            19 => IRs2::X19,
            20 => IRs2::X20,
            21 => IRs2::X21,
            22 => IRs2::X22,
            23 => IRs2::X23,
            24 => IRs2::X24,
            25 => IRs2::X25,
            26 => IRs2::X26,
            27 => IRs2::X27,
            28 => IRs2::X28,
            29 => IRs2::X29,
            30 => IRs2::X30,
            31 => IRs2::X31,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        Self::checked_from_u32(raw32 >> 20 & 0x1f).expect("Extracted value was larger than 5 bits")
    }
}

impl IRd {
    pub fn checked_from_u32(b: u32) -> Option<Self> {
        let res = match b {
            0..=31 => Self::wrapping_from_u32(b),
            _ => None?,
        };
        Some(res)
    }

    pub fn wrapping_from_u32(b: u32) -> Self {
        match b & 0x1f {
            0 => IRd::X0,
            1 => IRd::X1,
            2 => IRd::X2,
            3 => IRd::X3,
            4 => IRd::X4,
            5 => IRd::X5,
            6 => IRd::X6,
            7 => IRd::X7,
            8 => IRd::X8,
            9 => IRd::X9,
            10 => IRd::X10,
            11 => IRd::X11,
            12 => IRd::X12,
            13 => IRd::X13,
            14 => IRd::X14,
            15 => IRd::X15,
            16 => IRd::X16,
            17 => IRd::X17,
            18 => IRd::X18,
            19 => IRd::X19,
            20 => IRd::X20,
            21 => IRd::X21,
            22 => IRd::X22,
            23 => IRd::X23,
            24 => IRd::X24,
            25 => IRd::X25,
            26 => IRd::X26,
            27 => IRd::X27,
            28 => IRd::X28,
            29 => IRd::X29,
            30 => IRd::X30,
            31 => IRd::X31,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        Self::checked_from_u32(raw32 >> 7 & 0x1f).expect("Extracted value was larger than 5 bits")
    }
}

impl<T: RegType + Copy> RegFile<T> {
    pub fn get_rs1(&self, rs1: IRs1) -> T {
        unsafe { *self.reg.get_unchecked(rs1 as usize) }
    }

    pub fn get_rs2(&self, rs2: IRs2) -> T {
        unsafe { *self.reg.get_unchecked(rs2 as usize) }
    }

    pub fn set_rd(&mut self, rd: IRd, value: T) {
        unsafe {
            *self.reg.get_unchecked_mut(rd as usize) = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{IRd, IRs1, IRs2};

    #[test]
    fn test_basic_functionality() {
        use crate::reg::{IRd, IRs1, RegFile};
        let mut file = RegFile::<u32>::new();
        let rd = IRd::checked_from_u32(5).expect("Could not map number to register");
        let rs1 = IRs1::checked_from_u32(5).expect("Could not map number to register");

        file.set_rd(rd, 20);
        assert_eq!(file.get_rs1(rs1), 20);
    }

    #[test]
    fn test_no_write_x0() {
        use crate::reg::{IRd, IRs1, RegFile};
        let mut file = RegFile::<u32>::new();

        let rd = IRd::checked_from_u32(0).expect("Could not map number to register");
        let rs1 = IRs1::checked_from_u32(0).expect("Could not map number to register");

        file.set_rd(rd, 20);
        assert_eq!(file.get_rs1(rs1), 0);
        assert_eq!(file.reg[32], 20);
    }

    #[test]
    fn test_decode_regs() {
        let raw32_expected = [
            (0x003100b3, (IRd::X1, IRs1::X2, IRs2::X3)),
            (0x00628233, (IRd::X4, IRs1::X5, IRs2::X6)),
            (0x009403b3, (IRd::X7, IRs1::X8, IRs2::X9)),
            (0x00c58533, (IRd::X10, IRs1::X11, IRs2::X12)),
            (0x00f706b3, (IRd::X13, IRs1::X14, IRs2::X15)),
            (0x01288833, (IRd::X16, IRs1::X17, IRs2::X18)),
            (0x015a09b3, (IRd::X19, IRs1::X20, IRs2::X21)),
            (0x018b8b33, (IRd::X22, IRs1::X23, IRs2::X24)),
            (0x01bd0cb3, (IRd::X25, IRs1::X26, IRs2::X27)),
            (0x01ee8e33, (IRd::X28, IRs1::X29, IRs2::X30)),
            (0x00000fb3, (IRd::X31, IRs1::X0, IRs2::X0)),
        ];

        for (raw32, (rd, rs1, rs2)) in raw32_expected.into_iter() {
            assert_eq!(IRd::decode_raw32(raw32), rd);
            assert_eq!(IRs1::decode_raw32(raw32), rs1);
            assert_eq!(IRs2::decode_raw32(raw32), rs2);
        }
    }
}
