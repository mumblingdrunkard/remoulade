//! Module containing register file and register types, as well as functions and utilities to use
//! them effectively.

pub struct RegFile {
    reg: [u32; 33],
}

impl Default for RegFile {
    fn default() -> Self {
        Self::new()
    }
}

impl RegFile {
    pub fn new() -> Self {
        Self { reg: [0; 33] }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rs1 {
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

pub type Rs1Val = u32;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rs2 {
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

pub type Rs2Val = u32;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rd {
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

impl Rs1 {
    pub fn checked_from_u32(b: u32) -> Option<Self> {
        let res = match b {
            0..=31 => Self::wrapping_from_u32(b),
            _ => None?,
        };

        Some(res)
    }

    pub fn wrapping_from_u32(b: u32) -> Self {
        match b & 0x1f {
            0 => Rs1::X0,
            1 => Rs1::X1,
            2 => Rs1::X2,
            3 => Rs1::X3,
            4 => Rs1::X4,
            5 => Rs1::X5,
            6 => Rs1::X6,
            7 => Rs1::X7,
            8 => Rs1::X8,
            9 => Rs1::X9,
            10 => Rs1::X10,
            11 => Rs1::X11,
            12 => Rs1::X12,
            13 => Rs1::X13,
            14 => Rs1::X14,
            15 => Rs1::X15,
            16 => Rs1::X16,
            17 => Rs1::X17,
            18 => Rs1::X18,
            19 => Rs1::X19,
            20 => Rs1::X20,
            21 => Rs1::X21,
            22 => Rs1::X22,
            23 => Rs1::X23,
            24 => Rs1::X24,
            25 => Rs1::X25,
            26 => Rs1::X26,
            27 => Rs1::X27,
            28 => Rs1::X28,
            29 => Rs1::X29,
            30 => Rs1::X30,
            31 => Rs1::X31,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        Self::checked_from_u32(raw32 >> 15 & 0x1f).expect("Extracted value was larger than 5 bits")
    }
}

impl Rs2 {
    pub fn checked_from_u32(b: u32) -> Option<Self> {
        let res = match b {
            0..=31 => Self::wrapping_from_u32(b),
            _ => None?,
        };
        Some(res)
    }

    pub fn wrapping_from_u32(b: u32) -> Self {
        match b & 0x1f {
            0 => Rs2::X0,
            1 => Rs2::X1,
            2 => Rs2::X2,
            3 => Rs2::X3,
            4 => Rs2::X4,
            5 => Rs2::X5,
            6 => Rs2::X6,
            7 => Rs2::X7,
            8 => Rs2::X8,
            9 => Rs2::X9,
            10 => Rs2::X10,
            11 => Rs2::X11,
            12 => Rs2::X12,
            13 => Rs2::X13,
            14 => Rs2::X14,
            15 => Rs2::X15,
            16 => Rs2::X16,
            17 => Rs2::X17,
            18 => Rs2::X18,
            19 => Rs2::X19,
            20 => Rs2::X20,
            21 => Rs2::X21,
            22 => Rs2::X22,
            23 => Rs2::X23,
            24 => Rs2::X24,
            25 => Rs2::X25,
            26 => Rs2::X26,
            27 => Rs2::X27,
            28 => Rs2::X28,
            29 => Rs2::X29,
            30 => Rs2::X30,
            31 => Rs2::X31,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        Self::checked_from_u32(raw32 >> 20 & 0x1f).expect("Extracted value was larger than 5 bits")
    }
}

impl Rd {
    pub fn checked_from_u32(b: u32) -> Option<Self> {
        let res = match b {
            0..=31 => Self::wrapping_from_u32(b),
            _ => None?,
        };
        Some(res)
    }

    pub fn wrapping_from_u32(b: u32) -> Self {
        match b & 0x1f {
            0 => Rd::X0,
            1 => Rd::X1,
            2 => Rd::X2,
            3 => Rd::X3,
            4 => Rd::X4,
            5 => Rd::X5,
            6 => Rd::X6,
            7 => Rd::X7,
            8 => Rd::X8,
            9 => Rd::X9,
            10 => Rd::X10,
            11 => Rd::X11,
            12 => Rd::X12,
            13 => Rd::X13,
            14 => Rd::X14,
            15 => Rd::X15,
            16 => Rd::X16,
            17 => Rd::X17,
            18 => Rd::X18,
            19 => Rd::X19,
            20 => Rd::X20,
            21 => Rd::X21,
            22 => Rd::X22,
            23 => Rd::X23,
            24 => Rd::X24,
            25 => Rd::X25,
            26 => Rd::X26,
            27 => Rd::X27,
            28 => Rd::X28,
            29 => Rd::X29,
            30 => Rd::X30,
            31 => Rd::X31,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        Self::checked_from_u32(raw32 >> 7 & 0x1f).expect("Extracted value was larger than 5 bits")
    }
}

impl RegFile {
    pub fn get_rs1(&self, rs1: Rs1) -> Rs1Val {
        unsafe { *self.reg.get_unchecked(rs1 as usize) }
    }

    pub fn get_rs2(&self, rs2: Rs2) -> Rs2Val {
        unsafe { *self.reg.get_unchecked(rs2 as usize) }
    }

    pub fn set_rd(&mut self, rd: Rd, value: u32) {
        unsafe {
            *self.reg.get_unchecked_mut(rd as usize) = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Rd, Rs1, Rs2};

    #[test]
    fn test_basic_functionality() {
        use crate::reg::{Rd, RegFile, Rs1};
        let mut file = RegFile::new();
        let rd = Rd::checked_from_u32(5).expect("Could not map number to register");
        let rs1 = Rs1::checked_from_u32(5).expect("Could not map number to register");

        file.set_rd(rd, 20);
        assert_eq!(file.get_rs1(rs1), 20);
    }

    #[test]
    fn test_no_write_x0() {
        use crate::reg::{Rd, RegFile, Rs1};
        let mut file = RegFile::new();

        let rd = Rd::checked_from_u32(0).expect("Could not map number to register");
        let rs1 = Rs1::checked_from_u32(0).expect("Could not map number to register");

        file.set_rd(rd, 20);
        assert_eq!(file.get_rs1(rs1), 0);
        assert_eq!(file.reg[32], 20);
    }

    #[test]
    fn test_decode_regs() {
        let raw32_expected = [
            (0x003100b3, (Rd::X1, Rs1::X2, Rs2::X3)),
            (0x00628233, (Rd::X4, Rs1::X5, Rs2::X6)),
            (0x009403b3, (Rd::X7, Rs1::X8, Rs2::X9)),
            (0x00c58533, (Rd::X10, Rs1::X11, Rs2::X12)),
            (0x00f706b3, (Rd::X13, Rs1::X14, Rs2::X15)),
            (0x01288833, (Rd::X16, Rs1::X17, Rs2::X18)),
            (0x015a09b3, (Rd::X19, Rs1::X20, Rs2::X21)),
            (0x018b8b33, (Rd::X22, Rs1::X23, Rs2::X24)),
            (0x01bd0cb3, (Rd::X25, Rs1::X26, Rs2::X27)),
            (0x01ee8e33, (Rd::X28, Rs1::X29, Rs2::X30)),
            (0x00000fb3, (Rd::X31, Rs1::X0, Rs2::X0)),
        ];

        for (raw32, (rd, rs1, rs2)) in raw32_expected.into_iter() {
            assert_eq!(Rd::decode_raw32(raw32), rd);
            assert_eq!(Rs1::decode_raw32(raw32), rs1);
            assert_eq!(Rs2::decode_raw32(raw32), rs2);
        }
    }
}
