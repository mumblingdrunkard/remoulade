#[derive(Clone, Copy, Debug)]
/// A 12-bit wide signed integer
/// Format: `-[31]- | [30..=20]`
pub struct ITypeImmediate {
    value: i16,
}

impl ITypeImmediate {
    pub fn decode_raw32(raw32: u32) -> Self {
        let parts = [raw32 as i32 >> 20 & 0b11111111111111111111111111111111u32 as i32];
        let value = parts.into_iter().fold(0, |cur, part| cur | part) as i16;
        Self { value }
    }

    pub fn i32(&self) -> i32 {
        self.value as i32
    }
}

#[derive(Clone, Copy, Debug)]
/// A 12-bit wide signed integer
pub struct STypeImmediate {
    value: i16,
}

impl STypeImmediate {
    /// Format: `-[31]- | [30..=25] | [11..=7] `
    pub fn decode_raw32(raw32: u32) -> Self {
        let raw32 = raw32 as i32;
        let parts = [
            raw32 >> 20 & 0b11111111111111111111111111100000u32 as i32,
            raw32 >> 7 & 0b00000000000000000000000000011111u32 as i32,
        ];
        let value = parts.into_iter().fold(0, |cur, part| cur | part) as i16;
        Self { value }
    }

    pub fn i32(&self) -> i32 {
        self.value as i32
    }
}

#[derive(Clone, Copy, Debug)]
/// A 13-bit wide signed integer with the least significant bit cut off
pub struct BTypeImmediate {
    value: i16,
}

impl BTypeImmediate {
    /// Format: `-[31]- | [7] | [30..=25] | [11..=8] | 0`
    pub fn decode_raw32(raw32: u32) -> Self {
        let raw32 = raw32 as i32;
        let parts = [
            raw32 >> 20 & 0b11111111111111111111011111100000u32 as i32,
            raw32 << 4 & 0b00000000000000000000100000000000u32 as i32,
            raw32 >> 7 & 0b00000000000000000000000000011110u32 as i32,
        ];
        let value = parts.into_iter().fold(0, |cur, part| cur | part) as i16;
        Self { value }
    }

    pub fn i32(&self) -> i32 {
        self.value as i32
    }
}

#[derive(Clone, Copy, Debug)]
/// A 32-bit wide signed integer with the 12 least significant bits cut off
pub struct UTypeImmediate {
    value: i32,
}

impl UTypeImmediate {
    /// Format: `[31..=12] | - 0 -`
    pub fn decode_raw32(raw32: u32) -> Self {
        let raw32 = raw32 as i32;
        let parts = [raw32 & 0b11111111111111111111000000000000u32 as i32];
        let value = parts.into_iter().fold(0, |cur, part| cur | part);
        Self { value }
    }

    pub fn i32(&self) -> i32 {
        self.value
    }
}

#[derive(Clone, Copy, Debug)]
/// A 21-bit wide signed integer with the least significant bit cut off
pub struct JTypeImmediate {
    value: i32,
}

impl JTypeImmediate {
    /// Format: `-[31]- | [19..=12] | [20] | [30..=21] | 0`
    pub fn decode_raw32(raw32: u32) -> Self {
        let raw32 = raw32 as i32;
        let parts = [
            raw32 >> 20 & 0b11111111111100000000011111111110u32 as i32,
            raw32 & 0b00000000000011111111000000000000u32 as i32,
            raw32 >> 9 & 0b00000000000000000000100000000000u32 as i32,
        ];
        let value = parts.into_iter().fold(0, |cur, part| cur | part);
        Self { value }
    }

    pub fn i32(&self) -> i32 {
        self.value
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FenceFlags {
    flags: u8,
}

impl FenceFlags {
    const PI: u8 = 128;
    const PO: u8 = 64;
    const PR: u8 = 32;
    const PW: u8 = 16;
    const SI: u8 = 8;
    const SO: u8 = 4;
    const SR: u8 = 2;
    const SW: u8 = 1;

    pub fn decode_raw32(raw32: u32) -> Self {
        let flags = (raw32 >> 20) as u8;
        Self { flags }
    }

    pub fn pi(&self) -> bool {
        self.flags & Self::PI != 0
    }
    pub fn po(&self) -> bool {
        self.flags & Self::PO != 0
    }
    pub fn pr(&self) -> bool {
        self.flags & Self::PR != 0
    }
    pub fn pw(&self) -> bool {
        self.flags & Self::PW != 0
    }
    pub fn si(&self) -> bool {
        self.flags & Self::SI != 0
    }
    pub fn so(&self) -> bool {
        self.flags & Self::SO != 0
    }
    pub fn sr(&self) -> bool {
        self.flags & Self::SR != 0
    }
    pub fn sw(&self) -> bool {
        self.flags & Self::SW != 0
    }
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum FenceMode {
    None = 0b0000,
    Tso = 0b1000,
    Other,
}

impl FenceMode {
    fn decode_raw32(raw32: u32) -> Self {
        match raw32 >> 28 {
            0b0000 => FenceMode::None,
            0b1000 => FenceMode::Tso,
            _ => FenceMode::Other,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FenceInfo {
    mode: FenceMode,
    flags: FenceFlags,
}

impl FenceInfo {
    pub fn decode_raw32(raw32: u32) -> Self {
        let mode = FenceMode::decode_raw32(raw32);
        let flags = FenceFlags::decode_raw32(raw32);
        Self { mode, flags }
    }

    pub fn mode(&self) -> FenceMode {
        self.mode
    }
    pub fn flags(&self) -> FenceFlags {
        self.flags
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AmoAqrl {
    flags: u8,
}

impl AmoAqrl {
    const AQ: u8 = 2;
    const RL: u8 = 1;
}

impl AmoAqrl {
    pub fn decode_raw32(raw32: u32) -> Self {
        let flags = (raw32 >> 25) as u8;
        Self { flags }
    }

    pub fn aq(&self) -> bool {
        self.flags & AmoAqrl::AQ != 0
    }
    pub fn rl(&self) -> bool {
        self.flags & AmoAqrl::RL != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::inst::imm::{BTypeImmediate, ITypeImmediate, STypeImmediate};

    #[test]
    fn test_itype_decode() {
        let raw32_expected = [
            (0x3e800093, 1000),
            (0x7d008113, 2000),
            (0x7ff00093, 2047),
            (0x80000093, -2048),
        ];

        for (raw32, expected) in raw32_expected {
            let decoded = ITypeImmediate::decode_raw32(raw32);
            assert!(decoded.i32() == expected);
        }
    }

    #[test]
    fn test_stype_decode() {
        let raw32_expected = [
            (0x7e530fa3, 2047),
            (0x80730023, -2048),
            (0x2a430323, 678),
            (0xd4430d23, -678),
        ];

        for (raw32, expected) in raw32_expected {
            let decoded = STypeImmediate::decode_raw32(raw32);
            assert!(decoded.i32() == expected);
        }
    }

    #[test]
    fn test_btype_decode() {
        let raw32_expected = [
            (0x0000ce63, 28),
            (0x0000c663, 12),
            (0xfe00cee3, -4),
            (0xfe00c4e3, -24),
            (0xfe00c2e3, -28),
        ];

        for (raw32, expected) in raw32_expected {
            let decoded = BTypeImmediate::decode_raw32(raw32);
            assert!(decoded.i32() == expected);
        }
    }
}
