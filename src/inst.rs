pub mod imm;

use crate::{
    csr::Csr,
    inst::imm::{
        AmoAqrl, BTypeImmediate, FenceInfo, ITypeImmediate, JTypeImmediate, STypeImmediate,
        UTypeImmediate,
    },
    reg::{Rd, Rs1, Rs2},
};

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    Load,
    MiscMem,
    OpImm,
    Auipc,
    Store,
    Amo,
    Op,
    Lui,
    Branch,
    Jalr,
    Jal,
    System,
    Other,
}

impl OpCode {
    fn decode_raw32(raw32: u32) -> Self {
        use OpCode::*;
        match raw32 & 0x7f {
            0b0000011 => Load,
            0b0001111 => MiscMem,
            0b0010011 => OpImm,
            0b0010111 => Auipc,
            0b0100011 => Store,
            0b0101111 => Amo,
            0b0110011 => Op,
            0b0110111 => Lui,
            0b1100011 => Branch,
            0b1100111 => Jalr,
            0b1101111 => Jal,
            0b1110011 => System,
            _ => Other,
        }
    }
}

pub trait Fields16 {}

/// Adapter trait to extract instruction fields from 32-bit RISC-V instructions
pub trait Fields32 {
    // Discriminants

    fn opcode(&self) -> OpCode;
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

    fn rs1(&self) -> Rs1;
    fn rs2(&self) -> Rs2;
    fn rd(&self) -> Rd;
    fn csr(&self) -> Option<Csr>;
}

impl Fields32 for u32 {
    fn opcode(&self) -> OpCode {
        OpCode::decode_raw32(*self)
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

    fn rs1(&self) -> Rs1 {
        Rs1::decode_raw32(*self)
    }

    fn rs2(&self) -> Rs2 {
        Rs2::decode_raw32(*self)
    }

    fn rd(&self) -> Rd {
        Rd::decode_raw32(*self)
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

    // NOTE: Even though the immediate type for the immediate shifts have their own specialisation, we use a the same i-type immediate here and simply use a `::wrapping_shX` to perform the shift.
    //       This is the behaviour of the non-immediate shifts as well --- use only the 5 lower bits of rs2 to shift rs1.
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
}

#[repr(align(8))]
#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    UType {
        rd: Rd,
        u: UTypeImmediate,
        kind: UKind,
    },
    Jal {
        rd: Rd,
        j: JTypeImmediate,
    },
    IType {
        rd: Rd,
        rs1: Rs1,
        i: ITypeImmediate,
        kind: IKind,
    },
    BType {
        rs1: Rs1,
        rs2: Rs2,
        b: BTypeImmediate,
        kind: BKind,
    },
    SType {
        rs1: Rs1,
        rs2: Rs2,
        s: STypeImmediate,
        kind: SKind,
    },
    RType {
        rd: Rd,
        rs1: Rs1,
        rs2: Rs2,
        kind: RKind,
    },
    Fence {
        rd: Rd,
        rs1: Rs1,
        info: FenceInfo,
    },
    Ecall,
    Ebreak,
    CsrType {
        rd: Rd,
        rs1: Rs1,
        csr: Csr,
        kind: CsrKind,
    },
    AmoType {
        rd: Rd,
        rs1: Rs1,
        rs2: Rs2,
        aqrl: AmoAqrl,
        kind: AmoKind,
    },

    Illegal32 {
        raw32: u32,
    },

    Illegal16 {
        raw16: u16,
    },
}

impl Instruction {
    /// Tries to decode a 32-bit RISC-V instruction.
    /// Returns `Some(Instruction)` when decode is successful.
    /// Returns `None` when decoding produces an illegal instruction.
    fn decode_raw32_inner(raw32: u32) -> Option<Self> {
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

        let result = match OpCode::decode_raw32(raw32) {
            OpCode::Load => {
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

            OpCode::MiscMem => {
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

            OpCode::OpImm => {
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

            OpCode::Auipc => {
                let kind = UKind::Auipc;
                UType { rd, u, kind }
            }

            OpCode::Store => {
                let kind = match funct3 {
                    0b000 => SKind::Sb,
                    0b001 => SKind::Sh,
                    0b010 => SKind::Sw,
                    _ => None?,
                };
                SType { rs1, rs2, s, kind }
            }

            OpCode::Amo => {
                let aqrl = raw32.aqrl();
                let funct5 = raw32.funct5();
                let kind = match funct3 {
                    0b010 => match funct5 {
                        0b00010 if rs2 == Rs2::X0 => AmoKind::Lrw,
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

            OpCode::Op => {
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

            OpCode::Lui => {
                let kind = UKind::Lui;
                UType { rd, u, kind }
            }

            OpCode::Branch => {
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

            OpCode::Jalr => {
                let kind = IKind::Jalr;
                IType { rd, rs1, i, kind }
            }

            OpCode::Jal => Jal { rd, j },

            OpCode::System => {
                match funct3 {
                    0b000 if rd == Rd::X0 && rs1 == Rs1::X0 => match raw32.funct12() {
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

            OpCode::Other => None?,
        };

        Some(result)
    }

    pub fn decode_raw32(raw32: u32) -> Self {
        let default = Self::Illegal32 { raw32 };
        Self::decode_raw32_inner(raw32).unwrap_or(default)
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Illegal32 { raw32: 0 }
    }
}
