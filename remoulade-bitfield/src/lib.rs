// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright (C) 2024 mumblingdrunkard

mod helpers {
    #[macro_export]
    macro_rules! getter {
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, bool, $pos:literal) => {
            $(#[$a])*
            $vis fn $getter(&self) -> bool {
                self.0 & 1 << $pos != 0
            }
        };

        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, u8, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::getter!(@unsigned, $name, $t, $(#[$a])*, $vis, $getter, u8, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, u16, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::getter!(@unsigned, $name, $t, $(#[$a])*, $vis, $getter, u16, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, u32, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::getter!(@unsigned, $name, $t, $(#[$a])*, $vis, $getter, u32, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, u64, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::getter!(@unsigned, $name, $t, $(#[$a])*, $vis, $getter, u64, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, u128, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::getter!(@unsigned, $name, $t, $(#[$a])*, $vis, $getter, u128, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, i8, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::getter!(@signed, $name, $t, $(#[$a])*, $vis, $getter, i8, $ranges$(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, i16, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::getter!(@signed, $name, $t, $(#[$a])*, $vis, $getter, i16, $ranges$(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, i32, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::getter!(@signed, $name, $t, $(#[$a])*, $vis, $getter, i32, $ranges$(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, i64, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::getter!(@signed, $name, $t, $(#[$a])*, $vis, $getter, i64, $ranges$(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, i128, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::getter!(@signed, $name, $t, $(#[$a])*, $vis, $getter, i128, $ranges$(,$pad)?); };

        // Other type with impl From<$t>
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, $u:ty, [$($msb:literal:$lsb:literal)+] $(,$pad:literal)?) => {
            $(#[$a])*
            #[allow(redundant_semicolons)]
            $vis fn $getter(&self) -> $u {
                let val = 0;
                $(;
                    let mask = !((!0 as $t) << ($msb - $lsb + 1));
                    let val = val << ($msb - $lsb + 1) | (self.0 >> $lsb) & mask;
                )*
                $(;
                    let val = val << $pad;
                )?;
                <$u>::from(val)
            }
        };

        ( @unsigned, $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, $u:ty, [$($msb:literal:$lsb:literal),+] $(,$pad:literal)?) => {
            $(#[$a])*
            #[allow(redundant_semicolons)]
            $vis fn $getter(&self) -> $u {
                let val = 0;
                $(;
                    let mask = !((!0 << ($msb - $lsb + 1)) as $u);
                    let val = val << ($msb - $lsb + 1) | ((self.0 >> $lsb) as $u & mask) as $u;
                )*
                $(;
                    let val = val << $pad;
                )?;
                val
            }
        };

        ( @signed, $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $getter:ident, $u:ty, [$($msb:literal:$lsb:literal),+] $(,$pad:literal)?) => {
            $(#[$a])*
            #[allow(redundant_semicolons)]
            $vis fn $getter(&self) -> $u {
                let val = 0;
                let len = 0;
                $(;
                    let mask = !((!0 << ($msb - $lsb + 1)) as $u);
                    let val = val << ($msb - $lsb + 1) | ((self.0 >> $lsb) as $u & mask) as $u;
                    let len = len + ($msb - $lsb + 1);
                )*;
                let val = (val << (<$u>::BITS - len)) >> (<$u>::BITS - len $(-$pad)?);
                val
            }
        };
    }

    #[macro_export]
    macro_rules! setter {
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, bool, $pos:literal) => {
            $(#[$a])*
            #[allow(redundant_semicolons)]
            $vis fn $setter(&mut self, val: bool) {
                const UNSET: $t = !(1 << $pos);
                self.0 = (self.0 & UNSET) | (val as $t) << $pos;
            }
        };

        // Entry points for supported signed/unsigned types
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, u8, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::setter!(@signed_unsigned, $name, $t, $(#[$a])*, $vis, $setter, u8, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, u16, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::setter!(@signed_unsigned, $name, $t, $(#[$a])*, $vis, $setter, u16, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, u32, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::setter!(@signed_unsigned, $name, $t, $(#[$a])*, $vis, $setter, u32, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, u64, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::setter!(@signed_unsigned, $name, $t, $(#[$a])*, $vis, $setter, u64, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, u128, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::setter!(@signed_unsigned, $name, $t, $(#[$a])*, $vis, $setter, u128, $ranges $(,$pad)?); };

        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, i8, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::setter!(@signed_unsigned, $name, $t, $(#[$a])*, $vis, $setter, i8, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, i16, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::setter!(@signed_unsigned, $name, $t, $(#[$a])*, $vis, $setter, i16, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, i32, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::setter!(@signed_unsigned, $name, $t, $(#[$a])*, $vis, $setter, i32, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, i64, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::setter!(@signed_unsigned, $name, $t, $(#[$a])*, $vis, $setter, i64, $ranges $(,$pad)?); };
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, i128, $ranges:tt $(,$pad:literal)?) => { $crate::helpers::setter!(@signed_unsigned, $name, $t, $(#[$a])*, $vis, $setter, i128, $ranges $(,$pad)?); };

        // Support for any other types where impl From<$u> for $t
        ( $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, $u:ty, [$($msb:literal:$lsb:literal),+] $(,$pad:literal)?) => {
            $(#[$a])*
            #[allow(redundant_semicolons)]
            $vis fn $setter(&mut self, val: $u) {
                let _initial = <$t>::from(val);
                $(;
                    let mask = !(!0 << $pad);
                    debug_assert!(_initial & mask == 0, "<{}>::from({:?}) produced a value `{}` with a non-zero value in the padding when calling setter `{}`. I.e. one of the lower {} bits was non-zero after converting from the input type `{}` to the inner type `{}`.", std::stringify!($t), val, _initial, std::stringify!($setter), std::stringify!($pad), std::stringify!($u), std::stringify!($t));
                )?;

                let len = 0;
                $(;
                    let len = len + ($msb - $lsb + 1);
                )*;
                $(;
                    let len = len + $pad;
                )?;

                let _val = _initial << (<$t>::BITS - len);

                $(;
                    let len = $msb - $lsb + 1;
                    let mask = !(!0 << len);
                    let frag = _val >> (<$t>::BITS - len);
                    // Shift away the bits we extracted
                    let _val = _val << len;
                    // Shift mask and fragment into position
                    let mask = mask << $lsb;
                    let frag = frag << $lsb;
                    // Apply mask and fragment
                    self.0 = (self.0 & !mask) | (frag & mask);
                )*;
            }
        };

        ( @signed_unsigned, $name:ident, $t:ty, $(#[$a:meta])*, $vis:vis, $setter:ident, $u:ty, [$($msb:literal:$lsb:literal),+] $(,$pad:literal)?) => {
            $(#[$a])*
            #[allow(redundant_semicolons)]
            $vis fn $setter(&mut self, val: $u) {
                $(;
                    let mask = !(!0 << $pad);
                    debug_assert!(val & mask == 0, "One or more of the lower {} bits were non-zero.", std::stringify!($pad));
                )?;

                let len = 0;
                $(;
                    let len = len + ($msb - $lsb + 1);
                )*;
                $(;
                    let len = len + $pad;
                )?;


                let _val = val << (<$u>::BITS - len);

                $(;
                    let len = $msb - $lsb + 1;
                    let mask = !(!0 << len);
                    let frag = (_val >> (<$u>::BITS - len)) as $t;
                    // Shift away the bits we extracted
                    let _val = _val << len;
                    // Shift mask and fragment into position
                    let mask = mask << $lsb;
                    let frag = frag << $lsb;
                    self.0 = self.0 & !mask | frag & mask;
                )*;
            }
        };
    }

    #[macro_export]
    macro_rules! debug_fields {
        ( $name:ident,
            {
                $(
                    $(#[$a:meta])*
                    $vis:vis $getter:ident$(, $svis:vis $setter:ident)?: $u:tt @ $bits:tt $(<< $pad:expr)?,
                )*
            }
        ) => {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{} {{\n", std::stringify!($name)))?;
                $(
                    f.write_fmt(format_args!("    {}: {:?}\n", std::stringify!($getter), self.$getter()))?;
                )*
                f.write_fmt(format_args!("}}"))?;
                Ok(())
            }
        };

        ( $name:ident, { } ) => {};
    }

    #[macro_export]
    macro_rules! fields {
        ( $name:ident, $t:ty, { } ) => { };

        ( $name:ident, $t:ty,
            {
                $(#[$a:meta])*
                $vis:vis $getter:ident, $svis:vis $setter:ident: $u:tt @ $bits:tt $(<< $pad:expr)?,
                $($rest:tt)*
            }
        ) => {
            $crate::helpers::getter!($name, $t, $(#[$a])*, $vis, $getter, $u, $bits$(, $pad)?);
            $crate::helpers::setter!($name, $t, $(#[$a])*, $svis, $setter, $u, $bits$(, $pad)?);
            $crate::helpers::fields!($name, $t, { $($rest)* });
        };

        ( $name:ident, $t:ty,
            {
                $(#[$a:meta])*
                $vis:vis $getter:ident: $u:tt @ $bits:tt $(<< $pad:expr)?,
                $($rest:tt)*
            }
        ) => {
            $crate::helpers::getter!($name, $t, $(#[$a])*, $vis, $getter, $u, $bits$(, $pad)?);
            $crate::helpers::fields!($name, $t, { $($rest)* });
        };
    }

    pub use {debug_fields, fields, getter, setter};
}

#[macro_export]
macro_rules! bitfield {
    ( $(#[$a:meta])* $vis:vis struct $name:ident(u8) $($rest:tt)* ) => { $crate::bitfield!(@real, $(#[$a])*, $vis, $name, u8, $($rest)*); };
    ( $(#[$a:meta])* $vis:vis struct $name:ident(u16) $($rest:tt)* ) => { $crate::bitfield!(@real, $(#[$a])*, $vis, $name, u16, $($rest)*); };
    ( $(#[$a:meta])* $vis:vis struct $name:ident(u32) $($rest:tt)* ) => { $crate::bitfield!(@real, $(#[$a])*, $vis, $name, u32, $($rest)*); };
    ( $(#[$a:meta])* $vis:vis struct $name:ident(u64) $($rest:tt)* ) => { $crate::bitfield!(@real, $(#[$a])*, $vis, $name, u64, $($rest)*); };
    ( $(#[$a:meta])* $vis:vis struct $name:ident(u128) $($rest:tt)* ) => { $crate::bitfield!(@real, $(#[$a])*, $vis, $name, u128, $($rest)*); };

    ( @real, $(#[$a:meta])*, $vis:vis, $name:ident, $t:ty, with Debug $body:tt ) => {
        $(#[$a])*
        $vis struct $name(pub $t);
        impl std::fmt::Debug for $name {
            $crate::helpers::debug_fields!($name, $body);
        }
        impl $name { $crate::helpers::fields!($name, $t, $body); }
    };

    ( @real, $(#[$a:meta])*, $vis:vis, $name:ident, $t:ty, $body:tt ) => {
        $(#[$a])*
        $vis struct $name(pub $t);
        impl $name { $crate::helpers::fields!($name, $t, $body); }
    };
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Opcode {
        Load = 0b0000011,
        Loadfp = 0b0000111,
        Custom0 = 0b0001011,
        Miscmem = 0b0001111,
        Opimm = 0b0010011,
        Auipc = 0b0010111,
        Opimm32 = 0b0011011,
        Store = 0b0100011,
        Storefp = 0b0100111,
        Custom1 = 0b0101011,
        Amo = 0b0101111,
        Op = 0b0110011,
        Lui = 0b0110111,
        Op32 = 0b0111011,
        Madd = 0b1000011,
        Msub = 0b1000111,
        Nmsub = 0b1001011,
        Nmadd = 0b1001111,
        Opfp = 0b1010011,
        Custom2 = 0b1011011,
        Branch = 0b1100011,
        Jalr = 0b1100111,
        Jal = 0b1101111,
        System = 0b1110011,
        Custom3 = 0b1111011,
        Invalid = 0b0000000,
    }

    impl Opcode {
        pub const ALL: [Self; 26] = {
            use Opcode::*;
            [
                Load, Loadfp, Custom0, Miscmem, Opimm, Auipc, Opimm32, Store, Storefp, Custom1,
                Amo, Op, Lui, Op32, Madd, Msub, Nmsub, Nmadd, Opfp, Custom2, Branch, Jalr, Jal,
                System, Custom3, Invalid,
            ]
        };
    }

    impl From<u32> for Opcode {
        fn from(value: u32) -> Self {
            use Opcode::*;
            match value {
                0b0000011 => Load,
                0b0000111 => Loadfp,
                0b0001011 => Custom0,
                0b0001111 => Miscmem,
                0b0010011 => Opimm,
                0b0010111 => Auipc,
                0b0011011 => Opimm32,
                0b0100011 => Store,
                0b0100111 => Storefp,
                0b0101011 => Custom1,
                0b0101111 => Amo,
                0b0110011 => Op,
                0b0110111 => Lui,
                0b0111011 => Op32,
                0b1000011 => Madd,
                0b1000111 => Msub,
                0b1001011 => Nmsub,
                0b1001111 => Nmadd,
                0b1010011 => Opfp,
                0b1011011 => Custom2,
                0b1100011 => Branch,
                0b1100111 => Jalr,
                0b1101111 => Jal,
                0b1110011 => System,
                0b1111011 => Custom3,
                _ => Invalid,
            }
        }
    }

    impl From<Opcode> for u32 {
        fn from(value: Opcode) -> Self {
            value as u32
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    impl Rd {
        pub const ALL: [Self; 32] = {
            use Rd::*;
            [
                X0, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17,
                X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30, X31,
            ]
        };
    }

    impl From<u32> for Rd {
        fn from(value: u32) -> Self {
            use Rd::*;
            match value {
                0 => X0,
                1 => X1,
                2 => X2,
                3 => X3,
                4 => X4,
                5 => X5,
                6 => X6,
                7 => X7,
                8 => X8,
                9 => X9,
                10 => X10,
                11 => X11,
                12 => X12,
                13 => X13,
                14 => X14,
                15 => X15,
                16 => X16,
                17 => X17,
                18 => X18,
                19 => X19,
                20 => X20,
                21 => X21,
                22 => X22,
                23 => X23,
                24 => X24,
                25 => X25,
                26 => X26,
                27 => X27,
                28 => X28,
                29 => X29,
                30 => X30,
                31 => X31,
                _ => panic!(),
            }
        }
    }

    impl From<Rd> for u32 {
        fn from(value: Rd) -> u32 {
            match value {
                Rd::X0 => 0,
                _ => value as u32,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Rs {
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

    impl Rs {
        pub const ALL: [Self; 32] = {
            use Rs::*;
            [
                X0, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17,
                X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30, X31,
            ]
        };
    }

    impl From<u32> for Rs {
        fn from(value: u32) -> Self {
            use Rs::*;
            match value {
                0 => X0,
                1 => X1,
                2 => X2,
                3 => X3,
                4 => X4,
                5 => X5,
                6 => X6,
                7 => X7,
                8 => X8,
                9 => X9,
                10 => X10,
                11 => X11,
                12 => X12,
                13 => X13,
                14 => X14,
                15 => X15,
                16 => X16,
                17 => X17,
                18 => X18,
                19 => X19,
                20 => X20,
                21 => X21,
                22 => X22,
                23 => X23,
                24 => X24,
                25 => X25,
                26 => X26,
                27 => X27,
                28 => X28,
                29 => X29,
                30 => X30,
                31 => X31,
                _ => panic!(),
            }
        }
    }

    impl From<Rs> for u32 {
        fn from(value: Rs) -> u32 {
            match value {
                Rs::X0 => 0,
                _ => value as u32,
            }
        }
    }

    bitfield! {
        #[derive(Clone, Copy, Default)]
        pub struct BType(u32) with Debug {
            pub opcode, pub set_opcode: Opcode @ [6:0],
            pub funct3, pub set_funct3: u32 @ [14:12],
            pub rs1, pub set_rs1: Rs @ [19:15],
            pub rs2, pub set_rs2: Rs @ [24:20],
            pub imm, pub set_imm: i32 @ [31:31, 7:7, 30:25, 11:8] << 1,
        }
    }

    bitfield! {
        #[allow(unused)]
        #[derive(Clone, Copy, Default)]
        pub struct JType(u32) with Debug {
            pub opcode, pub set_opcode: Opcode @ [6:0],
            pub rd, pub set_rd: Rd @ [11:7],
            pub imm, pub set_imm: i32 @ [31:31, 19:12, 20:20, 30:21] << 1,
        }
    }

    #[test]
    fn test_jtype_opcode() {
        let mut j = JType(0);
        for op in Opcode::ALL {
            let before = (j.rd(), j.imm());
            j.set_opcode(op);
            assert_eq!(before, (j.rd(), j.imm()));
            assert_eq!(op, j.opcode());
        }
    }

    #[test]
    fn test_jtype_rd() {
        let mut j = JType(0);
        for rd in Rd::ALL {
            let before = (j.opcode(), j.imm());
            j.set_rd(rd);
            assert_eq!(before, (j.opcode(), j.imm()));
            assert_eq!(rd, j.rd());
        }
    }

    #[test]
    fn test_jtype_imm() {
        let mut j = JType(0);
        for imm in (-1 << 20..1 << 20).step_by(2) {
            let before = (j.opcode(), j.rd());
            j.set_imm(imm);
            assert_eq!(before, (j.opcode(), j.rd()));
            assert_eq!(imm, j.imm());
        }
    }

    #[test]
    fn test_btype_opcode() {
        let mut b = BType(0);
        for op in Opcode::ALL {
            let before = (b.funct3(), b.rs1(), b.rs2(), b.imm());
            b.set_opcode(op);
            assert_eq!(before, (b.funct3(), b.rs1(), b.rs2(), b.imm()));
            assert_eq!(op, b.opcode());
        }
    }

    #[test]
    fn test_btype_funct3() {
        let mut b = BType(0);
        for funct3 in 0..8 {
            let before = (b.opcode(), b.rs1(), b.rs2(), b.imm());
            b.set_funct3(funct3);
            assert_eq!(before, (b.opcode(), b.rs1(), b.rs2(), b.imm()));
            assert_eq!(funct3, b.funct3());
        }
    }

    #[test]
    fn test_btype_rs1() {
        let mut b = BType(0);
        for rs1 in Rs::ALL {
            let before = (b.opcode(), b.funct3(), b.rs2(), b.imm());
            b.set_rs1(rs1);
            assert_eq!(before, (b.opcode(), b.funct3(), b.rs2(), b.imm()));
            assert_eq!(rs1, b.rs1());
        }
    }

    #[test]
    fn test_btype_rs2() {
        let mut b = BType(0);
        for rs2 in Rs::ALL {
            let before = (b.opcode(), b.funct3(), b.rs1(), b.imm());
            b.set_rs2(rs2);
            assert_eq!(before, (b.opcode(), b.funct3(), b.rs1(), b.imm()));
            assert_eq!(rs2, b.rs2());
        }
    }

    #[test]
    fn test_btype_imm() {
        let mut b = BType(0);
        for imm in (-4096..4096).step_by(2) {
            let before = (b.opcode(), b.funct3(), b.rs1(), b.rs2());
            b.set_imm(imm);
            assert_eq!(before, (b.opcode(), b.funct3(), b.rs1(), b.rs2()));
            assert_eq!(imm, b.imm());
        }
    }
}
