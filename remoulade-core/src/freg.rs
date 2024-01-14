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

pub trait FRegType {}
impl FRegType for () {}
impl FRegType for f32 {}
impl FRegType for f64 {}

pub struct FRegFile<T: FRegType> {
    reg: [T; 33],
}

impl<T: Default + FRegType + Copy> Default for FRegFile<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default + FRegType + Copy> FRegFile<T> {
    pub fn new() -> Self {
        Self {
            reg: [T::default(); 33],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRs1 {
    F0 = 0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRs2 {
    F0 = 0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRd {
    F0 = 0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
}

impl FRs1 {
    pub fn checked_from_u32(b: u32) -> Option<Self> {
        let res = match b {
            0..=31 => Self::wrapping_from_u32(b),
            _ => None?,
        };

        Some(res)
    }

    pub fn wrapping_from_u32(b: u32) -> Self {
        match b & 0x1f {
            0 => FRs1::F0,
            1 => FRs1::F1,
            2 => FRs1::F2,
            3 => FRs1::F3,
            4 => FRs1::F4,
            5 => FRs1::F5,
            6 => FRs1::F6,
            7 => FRs1::F7,
            8 => FRs1::F8,
            9 => FRs1::F9,
            10 => FRs1::F10,
            11 => FRs1::F11,
            12 => FRs1::F12,
            13 => FRs1::F13,
            14 => FRs1::F14,
            15 => FRs1::F15,
            16 => FRs1::F16,
            17 => FRs1::F17,
            18 => FRs1::F18,
            19 => FRs1::F19,
            20 => FRs1::F20,
            21 => FRs1::F21,
            22 => FRs1::F22,
            23 => FRs1::F23,
            24 => FRs1::F24,
            25 => FRs1::F25,
            26 => FRs1::F26,
            27 => FRs1::F27,
            28 => FRs1::F28,
            29 => FRs1::F29,
            30 => FRs1::F30,
            31 => FRs1::F31,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        Self::checked_from_u32(raw32 >> 15 & 0x1f).expect("Extracted value was larger than 5 bits")
    }
}

impl FRs2 {
    pub fn checked_from_u32(b: u32) -> Option<Self> {
        let res = match b {
            0..=31 => Self::wrapping_from_u32(b),
            _ => None?,
        };
        Some(res)
    }

    pub fn wrapping_from_u32(b: u32) -> Self {
        match b & 0x1f {
            0 => FRs2::F0,
            1 => FRs2::F1,
            2 => FRs2::F2,
            3 => FRs2::F3,
            4 => FRs2::F4,
            5 => FRs2::F5,
            6 => FRs2::F6,
            7 => FRs2::F7,
            8 => FRs2::F8,
            9 => FRs2::F9,
            10 => FRs2::F10,
            11 => FRs2::F11,
            12 => FRs2::F12,
            13 => FRs2::F13,
            14 => FRs2::F14,
            15 => FRs2::F15,
            16 => FRs2::F16,
            17 => FRs2::F17,
            18 => FRs2::F18,
            19 => FRs2::F19,
            20 => FRs2::F20,
            21 => FRs2::F21,
            22 => FRs2::F22,
            23 => FRs2::F23,
            24 => FRs2::F24,
            25 => FRs2::F25,
            26 => FRs2::F26,
            27 => FRs2::F27,
            28 => FRs2::F28,
            29 => FRs2::F29,
            30 => FRs2::F30,
            31 => FRs2::F31,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        Self::checked_from_u32(raw32 >> 20 & 0x1f).expect("Extracted value was larger than 5 bits")
    }
}

impl FRd {
    pub fn checked_from_u32(b: u32) -> Option<Self> {
        let res = match b {
            0..=31 => Self::wrapping_from_u32(b),
            _ => None?,
        };
        Some(res)
    }

    pub fn wrapping_from_u32(b: u32) -> Self {
        match b & 0x1f {
            0 => FRd::F0,
            1 => FRd::F1,
            2 => FRd::F2,
            3 => FRd::F3,
            4 => FRd::F4,
            5 => FRd::F5,
            6 => FRd::F6,
            7 => FRd::F7,
            8 => FRd::F8,
            9 => FRd::F9,
            10 => FRd::F10,
            11 => FRd::F11,
            12 => FRd::F12,
            13 => FRd::F13,
            14 => FRd::F14,
            15 => FRd::F15,
            16 => FRd::F16,
            17 => FRd::F17,
            18 => FRd::F18,
            19 => FRd::F19,
            20 => FRd::F20,
            21 => FRd::F21,
            22 => FRd::F22,
            23 => FRd::F23,
            24 => FRd::F24,
            25 => FRd::F25,
            26 => FRd::F26,
            27 => FRd::F27,
            28 => FRd::F28,
            29 => FRd::F29,
            30 => FRd::F30,
            31 => FRd::F31,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        Self::checked_from_u32(raw32 >> 7 & 0x1f).expect("Extracted value was larger than 5 bits")
    }
}

impl<T: FRegType + Copy> FRegFile<T> {
    pub fn get_rs1(&self, rs1: FRs1) -> T {
        unsafe { *self.reg.get_unchecked(rs1 as usize) }
    }

    pub fn get_rs2(&self, rs2: FRs2) -> T {
        unsafe { *self.reg.get_unchecked(rs2 as usize) }
    }

    pub fn set_rd(&mut self, rd: FRd, value: T) {
        unsafe {
            *self.reg.get_unchecked_mut(rd as usize) = value;
        }
    }
}
