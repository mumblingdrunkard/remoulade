// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright (C) 2024 mumblingdrunkard

pub mod imm;

use std::{any::TypeId, marker::PhantomData};

use crate::{
    csr::Csr,
    freg::FRegType,
    inst::imm::{
        AmoAqrl, BTypeImmediate, FenceInfo, ITypeImmediate, JTypeImmediate, STypeImmediate,
        UTypeImmediate,
    },
    reg::{IRd, IRs1, IRs2, RegType},
};

#[derive(Copy, Clone, Debug)]
enum Opcode {
    Load,
    Loadfp,
    Miscmem,
    Opimm,
    Auipc,
    Opimm32,
    Store,
    Storefp,
    Amo,
    Op,
    Lui,
    Op32,
    Madd,
    Msub,
    Nmsub,
    Nmadd,
    Opfp,
    Branch,
    Jalr,
    Jal,
    System,
    Invalid,
}

impl Opcode {
    fn decode_raw32(raw32: u32) -> Self {
        use Opcode::*;
        match raw32 & 0x7f {
            0b0000011 => Load,
            0b0000111 => Loadfp,
            0b0001111 => Miscmem,
            0b0010011 => Opimm,
            0b0010111 => Auipc,
            0b0011011 => Opimm32,
            0b0100011 => Store,
            0b0100111 => Storefp,
            0b0101111 => Amo,
            0b0110011 => Op,
            0b0110111 => Lui,
            0b0111011 => Op32,
            0b1000011 => Madd,
            0b1000111 => Msub,
            0b1001011 => Nmsub,
            0b1001111 => Nmadd,
            0b1010011 => Opfp,
            0b1100011 => Branch,
            0b1100111 => Jalr,
            0b1101111 => Jal,
            0b1110011 => System,
            _ => Invalid,
        }
    }
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        Self::decode_raw32(value)
    }
}

/// Adapter trait to extract instruction fields from 32-bit RISC-V instructions
trait Fields32 {
    // Discriminants

    fn opcode(&self) -> Opcode;
    fn funct3(&self) -> u32;
    fn funct7(&self) -> u32;
    fn funct12(&self) -> u32;
    fn funct5(&self) -> u32;

    // Immediates

    fn i(&self) -> ITypeImmediate;
    fn b(&self) -> BTypeImmediate;
    fn s(&self) -> STypeImmediate;
    fn u(&self) -> UTypeImmediate;
    fn j(&self) -> JTypeImmediate;
    fn fence_info(&self) -> FenceInfo;
    fn aqrl(&self) -> AmoAqrl;

    // Registers

    fn rs1(&self) -> IRs1;
    fn rs2(&self) -> IRs2;
    fn rd(&self) -> IRd;
    fn csr(&self) -> Option<Csr>;
}

impl Fields32 for u32 {
    fn opcode(&self) -> Opcode {
        Opcode::decode_raw32(*self)
    }

    fn funct3(&self) -> u32 {
        *self >> 12 & 7
    }

    fn funct7(&self) -> u32 {
        *self >> 25
    }

    fn funct12(&self) -> u32 {
        *self >> 20
    }

    fn funct5(&self) -> u32 {
        *self >> 27
    }

    fn i(&self) -> ITypeImmediate {
        ITypeImmediate::decode_raw32(*self)
    }

    fn b(&self) -> BTypeImmediate {
        BTypeImmediate::decode_raw32(*self)
    }

    fn s(&self) -> STypeImmediate {
        STypeImmediate::decode_raw32(*self)
    }

    fn u(&self) -> UTypeImmediate {
        UTypeImmediate::decode_raw32(*self)
    }

    fn j(&self) -> JTypeImmediate {
        JTypeImmediate::decode_raw32(*self)
    }

    fn fence_info(&self) -> FenceInfo {
        FenceInfo::decode_raw32(*self)
    }

    fn aqrl(&self) -> AmoAqrl {
        AmoAqrl::decode_raw32(*self)
    }

    fn rs1(&self) -> IRs1 {
        IRs1::decode_raw32(*self)
    }

    fn rs2(&self) -> IRs2 {
        IRs2::decode_raw32(*self)
    }

    fn rd(&self) -> IRd {
        IRd::decode_raw32(*self)
    }

    fn csr(&self) -> Option<Csr> {
        Csr::decode_raw32(*self)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RKind {
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,

    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

#[derive(Copy, Clone, Debug)]
pub enum UKind {
    Lui,
    Auipc,
}

#[derive(Copy, Clone, Debug)]
pub enum BKind {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Copy, Clone, Debug)]
pub enum IKind {
    Jalr,

    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,

    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,

    Slli,
    Srli,
    Srai,

    Fencei,
}

#[derive(Copy, Clone, Debug)]
pub enum CsrKind {
    Csrrw,
    Csrrs,
    Csrrc,

    // NOTE: Immediate types interpret the 5-bit register selector `rs1` as an unsigned integer and uses that instead of `reg[rs1]`.
    Csrrwi,
    Csrrsi,
    Csrrci,
}

#[derive(Copy, Clone, Debug)]
pub enum SKind {
    Sb,
    Sh,
    Sw,
}

#[derive(Copy, Clone, Debug)]
pub enum AmoKind {
    // NOTE: Lrw requires rs2 be Rs2::X0.
    //       (I.e. there is no rs2)
    //       We verify this at decode time to avoid adding another variant.
    Lrw,
    Scw,
    Amoswapw,
    Amoaddw,
    Amoxorw,
    Amoandw,
    Amoorw,
    Amominw,
    Amomaxw,
    Amominuw,
    Amomaxuw,

    Lrd,
    Scd,
    Amoswapd,
    Amoaddd,
    Amoxord,
    Amoandd,
    Amoord,
    Amomind,
    Amomaxd,
    Amominud,
    Amomaxud,
}

#[repr(align(4))]
pub enum Compressed {
    Cr { rd_rs1: IRd, rs2: IRs2 },
}

#[repr(align(8))]
#[derive(Clone, Copy, Debug)]
pub enum Instruction<I: RegType, const M: bool, const A: bool, F: FRegType> {
    UType {
        rd: IRd,
        u: UTypeImmediate,
        kind: UKind,
    },
    Jal {
        rd: IRd,
        j: JTypeImmediate,
    },
    IType {
        rd: IRd,
        rs1: IRs1,
        i: ITypeImmediate,
        kind: IKind,
    },
    BType {
        rs1: IRs1,
        rs2: IRs2,
        b: BTypeImmediate,
        kind: BKind,
    },
    SType {
        rs1: IRs1,
        rs2: IRs2,
        s: STypeImmediate,
        kind: SKind,
    },
    RType {
        rd: IRd,
        rs1: IRs1,
        rs2: IRs2,
        kind: RKind,
    },
    Fence {
        rd: IRd,
        rs1: IRs1,
        info: FenceInfo,
    },
    Ecall,
    Ebreak,
    CsrType {
        rd: IRd,
        rs1: IRs1,
        csr: Csr,
        kind: CsrKind,
    },
    AmoType {
        rd: IRd,
        rs1: IRs1,
        rs2: IRs2,
        aqrl: AmoAqrl,
        kind: AmoKind,
    },

    Illegal32 {
        raw32: u32,
    },

    Illegal16 {
        raw16: u16,
    },

    Unused {
        _pdi: PhantomData<I>,
        _pdf: PhantomData<F>,
    },
}

impl<I: 'static + RegType, const M: bool, const A: bool, F: FRegType> Instruction<I, M, A, F> {
    /// Tries to decode a 32-bit RISC-V instruction.
    /// Returns `Some(Instruction)` when decode is successful.
    /// Returns `None` when decoding produces an illegal instruction.
    fn decode_raw32_inner(raw32: u32) -> Option<Self> {
        // I'd like these to be const, but we can't use generic parameters in const contexts like this yet
        let rv64 =
            TypeId::of::<I>() == TypeId::of::<u64>() || TypeId::of::<I>() == TypeId::of::<u128>();
        let rv128 = TypeId::of::<I>() == TypeId::of::<u128>();

        use Instruction::*;
        let rd = raw32.rd();
        let rs1 = raw32.rs1();
        let rs2 = raw32.rs2();
        let i = raw32.i();
        let j = raw32.j();
        let b = raw32.b();
        let s = raw32.s();
        let u = raw32.u();
        let funct3 = raw32.funct3();
        let funct7 = raw32.funct7();

        let result = match Opcode::decode_raw32(raw32) {
            Opcode::Load => {
                let kind = match funct3 {
                    0b000 => IKind::Lb,
                    0b001 => IKind::Lh,
                    0b010 => IKind::Lw,
                    0b100 => IKind::Lbu,
                    0b101 => IKind::Lhu,
                    _ => None?,
                };
                IType { rd, rs1, i, kind }
            }

            Opcode::Miscmem => {
                let info = raw32.fence_info();
                match funct3 {
                    0b000 => Fence { rd, rs1, info },
                    0b001 => {
                        let kind = IKind::Fencei;
                        IType { rd, rs1, i, kind }
                    }
                    _ => None?,
                }
            }

            Opcode::Opimm => {
                let kind = match (funct3, funct7) {
                    (0b000, 0b0000000) => IKind::Addi,
                    (0b010, 0b0000000) => IKind::Slti,
                    (0b011, 0b0000000) => IKind::Sltiu,
                    (0b100, 0b0000000) => IKind::Xori,
                    (0b110, 0b0000000) => IKind::Ori,
                    (0b111, 0b0000000) => IKind::Andi,
                    (0b001, 0b0000000) => IKind::Slli,
                    (0b101, 0b0000000) => IKind::Srli,
                    (0b101, 0b0100000) => IKind::Srai,
                    _ => None?,
                };
                IType { rd, rs1, i, kind }
            }

            Opcode::Auipc => {
                let kind = UKind::Auipc;
                UType { rd, u, kind }
            }

            Opcode::Store => {
                let kind = match funct3 {
                    0b000 => SKind::Sb,
                    0b001 => SKind::Sh,
                    0b010 => SKind::Sw,
                    _ => None?,
                };
                SType { rs1, rs2, s, kind }
            }

            Opcode::Amo => {
                let aqrl = raw32.aqrl();
                let funct5 = raw32.funct5();
                let kind = match funct3 {
                    0b010 => match funct5 {
                        0b00010 if rs2 == IRs2::X0 => AmoKind::Lrw,
                        0b00011 => AmoKind::Scw,
                        0b00001 => AmoKind::Amoswapw,
                        0b00000 => AmoKind::Amoaddw,
                        0b00100 => AmoKind::Amoxorw,
                        0b01100 => AmoKind::Amoandw,
                        0b01000 => AmoKind::Amoorw,
                        0b10000 => AmoKind::Amominw,
                        0b10100 => AmoKind::Amomaxw,
                        0b11000 => AmoKind::Amominuw,
                        0b11100 => AmoKind::Amomaxuw,
                        _ => None?,
                    },
                    _ => None?,
                };
                AmoType {
                    rd,
                    rs1,
                    rs2,
                    aqrl,
                    kind,
                }
            }

            Opcode::Op => {
                let kind = match (funct3, funct7) {
                    (0b000, 0b0000000) => RKind::Add,
                    (0b000, 0b0100000) => RKind::Sub,
                    (0b001, 0b0000000) => RKind::Sll,
                    (0b010, 0b0000000) => RKind::Slt,
                    (0b011, 0b0000000) => RKind::Sltu,
                    (0b100, 0b0000000) => RKind::Xor,
                    (0b101, 0b0000000) => RKind::Srl,
                    (0b101, 0b0100000) => RKind::Sra,
                    (0b110, 0b0000000) => RKind::Or,
                    (0b111, 0b0000000) => RKind::And,

                    (0b000, 0b0000001) => RKind::Mul,
                    (0b001, 0b0000001) => RKind::Mulh,
                    (0b010, 0b0000001) => RKind::Mulhsu,
                    (0b011, 0b0000001) => RKind::Mulhu,
                    (0b100, 0b0000001) => RKind::Div,
                    (0b101, 0b0000001) => RKind::Divu,
                    (0b110, 0b0000001) => RKind::Rem,
                    (0b111, 0b0000001) => RKind::Remu,

                    _ => None?,
                };
                RType { rd, rs1, rs2, kind }
            }

            Opcode::Lui => {
                let kind = UKind::Lui;
                UType { rd, u, kind }
            }

            Opcode::Branch => {
                let kind = match funct3 {
                    0b000 => BKind::Beq,
                    0b001 => BKind::Bne,
                    0b100 => BKind::Blt,
                    0b101 => BKind::Bge,
                    0b110 => BKind::Bltu,
                    0b111 => BKind::Bgeu,
                    _ => None?,
                };
                BType { rs1, rs2, b, kind }
            }

            Opcode::Jalr => {
                let kind = IKind::Jalr;
                IType { rd, rs1, i, kind }
            }

            Opcode::Jal => Jal { rd, j },

            Opcode::System => {
                match funct3 {
                    0b000 if rd == IRd::X0 && rs1 == IRs1::X0 => match raw32.funct12() {
                        0b000000000000 => Ecall,
                        0b000000000001 => Ebreak,
                        _ => None?,
                    },

                    0b001..=0b011 | 0b101..=0b111 => {
                        // NOTE: "Attempts to access a non-existent CSR raise an illegal instruction exception."
                        //       --- RISC-V Privileged specification, p. 6
                        //       Thus, we can verify at decode time whether this should decode into an invalid instruction.
                        let csr = raw32.csr()?;

                        let kind = match funct3 {
                            0b001 => CsrKind::Csrrw,
                            0b010 => CsrKind::Csrrs,
                            0b011 => CsrKind::Csrrc,
                            0b101 => CsrKind::Csrrwi,
                            0b110 => CsrKind::Csrrsi,
                            0b111 => CsrKind::Csrrci,
                            // NOTE: Unreachable because we've already matched both ranges
                            _ => unsafe { std::hint::unreachable_unchecked() },
                        };
                        CsrType { rd, rs1, csr, kind }
                    }

                    _ => None?,
                }
            }

            Opcode::Invalid => None?,
            Opcode::Loadfp => todo!(),
            Opcode::Opimm32 => todo!(),
            Opcode::Storefp => todo!(),
            Opcode::Op32 => todo!(),
            Opcode::Madd => todo!(),
            Opcode::Msub => todo!(),
            Opcode::Nmsub => todo!(),
            Opcode::Nmadd => todo!(),
            Opcode::Opfp => todo!(),
        };

        Some(result)
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        let default = Self::Illegal32 { raw32 };
        Self::decode_raw32_inner(raw32).unwrap_or(default)
    }
}

impl<I: RegType, const M: bool, const A: bool, F: FRegType> Default for Instruction<I, M, A, F> {
    fn default() -> Self {
        Self::Illegal32 { raw32: 0 }
    }
}
