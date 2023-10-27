#[derive(Copy, Clone, Debug)]
/// CSR registers.
///
/// Variants are documented as ``Number | Privilege | Description``
pub enum Csr {
    // Unprivileged Floating-Point CSRs
    /// ``0x001 | URW | Floating-Point Accrued Exceptions.``
    Fflags,
    /// ``0x002 | URW | Floating-Point Dynamic Rounding Mode.``
    Frm,
    /// ``0x003 | URW | Floating-Point Control and Status Register (`Frm + Fflags`).``
    Fcsr,

    // Unprivileged Counters/Timers
    /// ``0xC00 | URO | Cycle counter for RDCYCLE instruction.``
    Cycle,
    /// ``0xC01 | URO | Timer for RDTIME instruction.``
    Time,
    /// ``0xC02 | URO | Timer for RDINSTRET instruction.``
    Instret,
    /// ``0xC03 | URO | Performance-monitoring counter.``
    Hpmcounter3,
    /// ``0xC04 | URO | Performance-monitoring counter.``
    Hpmcounter4,
    /// ``0xC05 | URO | Performance-monitoring counter.``
    Hpmcounter5,
    /// ``0xC06 | URO | Performance-monitoring counter.``
    Hpmcounter6,
    /// ``0xC07 | URO | Performance-monitoring counter.``
    Hpmcounter7,
    /// ``0xC08 | URO | Performance-monitoring counter.``
    Hpmcounter8,
    /// ``0xC09 | URO | Performance-monitoring counter.``
    Hpmcounter9,
    /// ``0xC0A | URO | Performance-monitoring counter.``
    Hpmcounter10,
    /// ``0xC0B | URO | Performance-monitoring counter.``
    Hpmcounter11,
    /// ``0xC0C | URO | Performance-monitoring counter.``
    Hpmcounter12,
    /// ``0xC0D | URO | Performance-monitoring counter.``
    Hpmcounter13,
    /// ``0xC0E | URO | Performance-monitoring counter.``
    Hpmcounter14,
    /// ``0xC0F | URO | Performance-monitoring counter.``
    Hpmcounter15,
    /// ``0xC10 | URO | Performance-monitoring counter.``
    Hpmcounter16,
    /// ``0xC11 | URO | Performance-monitoring counter.``
    Hpmcounter17,
    /// ``0xC12 | URO | Performance-monitoring counter.``
    Hpmcounter18,
    /// ``0xC13 | URO | Performance-monitoring counter.``
    Hpmcounter19,
    /// ``0xC14 | URO | Performance-monitoring counter.``
    Hpmcounter20,
    /// ``0xC15 | URO | Performance-monitoring counter.``
    Hpmcounter21,
    /// ``0xC16 | URO | Performance-monitoring counter.``
    Hpmcounter22,
    /// ``0xC17 | URO | Performance-monitoring counter.``
    Hpmcounter23,
    /// ``0xC18 | URO | Performance-monitoring counter.``
    Hpmcounter24,
    /// ``0xC19 | URO | Performance-monitoring counter.``
    Hpmcounter25,
    /// ``0xC1A | URO | Performance-monitoring counter.``
    Hpmcounter26,
    /// ``0xC1B | URO | Performance-monitoring counter.``
    Hpmcounter27,
    /// ``0xC1C | URO | Performance-monitoring counter.``
    Hpmcounter28,
    /// ``0xC1D | URO | Performance-monitoring counter.``
    Hpmcounter29,
    /// ``0xC1E | URO | Performance-monitoring counter.``
    Hpmcounter30,
    /// ``0xC1F | URO | Performance-monitoring counter.``
    Hpmcounter31,

    /// ``0xC80 | URO | Upper 32 bits of `Cycle`, RV32 only.``
    Cycleh,
    /// ``0xC81 | URO | Upper 32 bits of `Time`, RV32 only.``
    Timeh,
    /// ``0xC82 | URO | Upper 32 bits of `Instret`, RV32 only.``
    Instreth,
    /// ``0xC83 | URO | Upper 32 bits of `Hpmcounter3`, RV32 only.``
    Hpmcounter3h,
    /// ``0xC84 | URO | Upper 32 bits of `Hpmcounter4`, RV32 only.``
    Hpmcounter4h,
    /// ``0xC85 | URO | Upper 32 bits of `Hpmcounter5`, RV32 only.``
    Hpmcounter5h,
    /// ``0xC86 | URO | Upper 32 bits of `Hpmcounter6`, RV32 only.``
    Hpmcounter6h,
    /// ``0xC87 | URO | Upper 32 bits of `Hpmcounter7`, RV32 only.``
    Hpmcounter7h,
    /// ``0xC88 | URO | Upper 32 bits of `Hpmcounter8`, RV32 only.``
    Hpmcounter8h,
    /// ``0xC89 | URO | Upper 32 bits of `Hpmcounter9`, RV32 only.``
    Hpmcounter9h,
    /// ``0xC8A | URO | Upper 32 bits of `Hpmcounter10`, RV32 only.``
    Hpmcounter10h,
    /// ``0xC8B | URO | Upper 32 bits of `Hpmcounter11`, RV32 only.``
    Hpmcounter11h,
    /// ``0xC8C | URO | Upper 32 bits of `Hpmcounter12`, RV32 only.``
    Hpmcounter12h,
    /// ``0xC8D | URO | Upper 32 bits of `Hpmcounter13`, RV32 only.``
    Hpmcounter13h,
    /// ``0xC8E | URO | Upper 32 bits of `Hpmcounter14`, RV32 only.``
    Hpmcounter14h,
    /// ``0xC8F | URO | Upper 32 bits of `Hpmcounter15`, RV32 only.``
    Hpmcounter15h,
    /// ``0xC90 | URO | Upper 32 bits of `Hpmcounter16`, RV32 only.``
    Hpmcounter16h,
    /// ``0xC91 | URO | Upper 32 bits of `Hpmcounter17`, RV32 only.``
    Hpmcounter17h,
    /// ``0xC92 | URO | Upper 32 bits of `Hpmcounter18`, RV32 only.``
    Hpmcounter18h,
    /// ``0xC93 | URO | Upper 32 bits of `Hpmcounter19`, RV32 only.``
    Hpmcounter19h,
    /// ``0xC94 | URO | Upper 32 bits of `Hpmcounter20`, RV32 only.``
    Hpmcounter20h,
    /// ``0xC95 | URO | Upper 32 bits of `Hpmcounter21`, RV32 only.``
    Hpmcounter21h,
    /// ``0xC96 | URO | Upper 32 bits of `Hpmcounter22`, RV32 only.``
    Hpmcounter22h,
    /// ``0xC97 | URO | Upper 32 bits of `Hpmcounter23`, RV32 only.``
    Hpmcounter23h,
    /// ``0xC98 | URO | Upper 32 bits of `Hpmcounter24`, RV32 only.``
    Hpmcounter24h,
    /// ``0xC99 | URO | Upper 32 bits of `Hpmcounter25`, RV32 only.``
    Hpmcounter25h,
    /// ``0xC9A | URO | Upper 32 bits of `Hpmcounter26`, RV32 only.``
    Hpmcounter26h,
    /// ``0xC9B | URO | Upper 32 bits of `Hpmcounter27`, RV32 only.``
    Hpmcounter27h,
    /// ``0xC9C | URO | Upper 32 bits of `Hpmcounter28`, RV32 only.``
    Hpmcounter28h,
    /// ``0xC9D | URO | Upper 32 bits of `Hpmcounter29`, RV32 only.``
    Hpmcounter29h,
    /// ``0xC9E | URO | Upper 32 bits of `Hpmcounter30`, RV32 only.``
    Hpmcounter30h,
    /// ``0xC9F | URO | Upper 32 bits of `Hpmcounter31`, RV32 only.``
    Hpmcounter31h,

    // Supervisor Trap Setup
    /// ``0x100 | SRW | Supervisor status register.``
    Sstatus,
    /// ``0x104 | SRW | Supervisor interrupt-enable register.``
    Ssie,
    /// ``0x105 | SRW | Supervisor trap handler base address.``
    Stvec,
    /// ``0x106 | SRW | Supervisor counter enable.``
    Scounteren,

    // Supervisor Configuration
    /// ``0x10A | SRW | Supervisor environment configuration register.``
    Senvcfg,

    // Supervisor Trap Handling
    /// ``0x140 | SRW | Supervisor register for supervisor trap handlers.``
    Sscratch,
    /// ``0x141 | SRW | Supervisor exception program counter.``
    Sepc,
    /// ``0x142 | SRW | Supervisor trap cause.``
    Scause,
    /// ``0x143 | SRW | Supervisor bad address or instruction.``
    Stval,
    /// ``0x144 | SRW | Supervisor interrupt pending.``
    Sip,

    // Supervisor Protection and Translation
    /// ``0x180 | SRW | Supervisor address translation and protection.``
    Satp,

    // Debug/Trace Registers
    /// ``0x5A8 | SRW | Supervisor-mode context register.``
    Scontext,

    // Hypervisor Trap Setup
    /// ``0x600 | HRW | Hypervisor status register.``
    Hstatus,
    /// ``0x602 | HRW | Hypervisor exception delegation register.``
    Hedeleg,
    /// ``0x603 | HRW | Hypervisor interrupt delegation register.``
    Hideleg,
    /// ``0x604 | HRW | Hypervisor interrupt-enable register.``
    Hie,
    /// ``0x606 | HRW | Hypervisor counter enable.``
    Hcounteren,
    /// ``0x607 | HRW | Hypervisor guest external interrupt-enable register.``
    Hgeie,

    // Hypervisor Trap Handling
    /// ``0x643 | HRW | Hypervisor bad guest physical address.``
    Htval,
    /// ``0x644 | HRW | Hypervisor interrupt pending.``
    Hip,
    /// ``0x645 | HRW | Hypervisor virtual interrupt pending.``
    Hvip,
    /// ``0x64A | HRW | Hypervisor trap instruction (transformed).``
    Htinst,
    /// ``0xE12 | HRO | Hypervisor guest external interrupt pending.``
    Hgeip,

    // Hypervisor Configuration
    /// ``0x60A | HRW | Hypervisor environment configuration register.``
    Henvcfg,
    /// ``0x61A | HRW | Additional hypervisor env. conf. register, RV32 only.``
    Henvcfgh,

    // Debug/Trace Registers
    /// ``0x6A8 | HRW | Hypervisor-mode context register.``
    Hcontext,

    // Hypervisor Counter/Timer Virtualization Registers
    /// ``0x605 | HRW | Delta for VS/VU-mode timer.``
    Htimedelta,
    /// ``0x615 | HRW | Upper 32 bits of `Htimedelta`, HSXLEN=32 only.``
    Htimedeltah,

    // Virtual Supervisor Registers
    /// ``0x200 | HRW | Virtual supervisor status register.``
    Vsstatus,
    /// ``0x204 | HRW | Virtual supervisor interrupt-enable register.``
    Vsie,
    /// ``0x205 | HRW | Virtual supervisor trap handler base address.``
    Vstvec,
    /// ``0x240 | HRW | Virtual supervisor scratch register.``
    Vsscratch,
    /// ``0x241 | HRW | Virtual supervisor exception program counter.``
    Vsepc,
    /// ``0x242 | HRW | Virtual supervisor trap cause.``
    Vscause,
    /// ``0x243 | HRW | Virtual supervisor bad address or instruction.``
    Vstval,
    /// ``0x244 | HRW | Virtual supervisor interrupt pending.``
    Vsip,
    /// ``0x280 | HRW | Virtual supervisor address translation and protection.``
    Vsatp,

    // Machine Information Registers
    /// ``0xF11 | MRO | Vendor ID.``
    Mvendorid,
    /// ``0xF12 | MRO | Architecture ID.``
    Marchid,
    /// ``0xF13 | MRO | Implementation ID.``
    Mimpid,
    /// ``0xF14 | MRO | Hardware thread ID.``
    Mhartid,
    /// ``0xF15 | MRO | Pointer to configuration data structure.``
    Mconfigptr,

    // Machine Trap Setup
    /// ``0x300 | MRW | Machine status register.``
    Mstatus,
    /// ``0x301 | MRW | ISA and extensions.``
    Misa,
    /// ``0x302 | MRW | Machine exception delegation register.``
    Medeleg,
    /// ``0x303 | MRW | Machine interrupt delegation register.``
    Mideleg,
    /// ``0x304 | MRW | Machine interrupt-enable register.``
    Mie,
    /// ``0x305 | MRW | Machine trap-handler base address.``
    Mtvec,
    /// ``0x306 | MRW | Machine counter enable.``
    Mcounteren,
    /// ``0x310 | MRW | Additional machine status register, RV32 only.``
    Mstatush,

    // Machine Trap Handling
    /// ``0x340 | MRW | Scratch register for machine trap handlers.``
    Mscratch,
    /// ``0x341 | MRW | Machine exception program counter.``
    Mepc,
    /// ``0x342 | MRW | Machine trap cause.``
    Mcause,
    /// ``0x343 | MRW | Machine bad address or instruction.``
    Mtval,
    /// ``0x344 | MRW | Machine interrupt pending.``
    Mip,
    /// ``0x34A | MRW | Machine trap instruction (transformed).``
    Mtinst,
    /// ``0x34B | MRW | Machine bad guest physical address.``
    Mtval2,

    // Machine Configuration
    /// ``0x30A | MRW | Machine environment configuration register.``
    Menvcfg,
    /// ``0x31A | MRW | Additional machine env. conf. register, RV32 only.``
    Menvcfgh,
    /// ``0x747 | MRW | Machine security configuration register.``
    Mseccfg,
    /// ``0x757 | MRW | Additional machine security conf. register, RV32 only.``
    Mseccfgh,

    // Machine Memory Protection
    /// ``0x3A0 | MRW | Physical memory protection configuration.``
    Pmpcfg0,
    /// ``0x3A1 | MRW | Physical memory protection configuration, RV32 only.``
    Pmpcfg1,
    /// ``0x3A2 | MRW | Physical memory protection configuration.``
    Pmpcfg2,
    /// ``0x3A3 | MRW | Physical memory protection configuration, RV32 only.``
    Pmpcfg3,
    /// ``0x3A4 | MRW | Physical memory protection configuration.``
    Pmpcfg4,
    /// ``0x3A5 | MRW | Physical memory protection configuration, RV32 only.``
    Pmpcfg5,
    /// ``0x3A6 | MRW | Physical memory protection configuration.``
    Pmpcfg6,
    /// ``0x3A7 | MRW | Physical memory protection configuration, RV32 only.``
    Pmpcfg7,
    /// ``0x3A8 | MRW | Physical memory protection configuration.``
    Pmpcfg08,
    /// ``0x3A9 | MRW | Physical memory protection configuration, RV32 only.``
    Pmpcfg09,
    /// ``0x3AA | MRW | Physical memory protection configuration.``
    Pmpcfg10,
    /// ``0x3AB | MRW | Physical memory protection configuration, RV32 only.``
    Pmpcfg11,
    /// ``0x3AC | MRW | Physical memory protection configuration.``
    Pmpcfg12,
    /// ``0x3AD | MRW | Physical memory protection configuration, RV32 only.``
    Pmpcfg13,
    /// ``0x3AE | MRW | Physical memory protection configuration.``
    Pmpcfg14,
    /// ``0x3AF | MRW | Physical memory protection configuration, RV32 only.``
    Pmpcfg15,

    /// ``0x3B0 | MRW | Physical memory protection address register.``
    Pmpaddr0,
    /// ``0x3B1 | MRW | Physical memory protection address register.``
    Pmpaddr1,
    /// ``0x3B2 | MRW | Physical memory protection address register.``
    Pmpaddr2,
    /// ``0x3B3 | MRW | Physical memory protection address register.``
    Pmpaddr3,
    /// ``0x3B4 | MRW | Physical memory protection address register.``
    Pmpaddr4,
    /// ``0x3B5 | MRW | Physical memory protection address register.``
    Pmpaddr5,
    /// ``0x3B6 | MRW | Physical memory protection address register.``
    Pmpaddr6,
    /// ``0x3B7 | MRW | Physical memory protection address register.``
    Pmpaddr7,
    /// ``0x3B8 | MRW | Physical memory protection address register.``
    Pmpaddr8,
    /// ``0x3B9 | MRW | Physical memory protection address register.``
    Pmpaddr9,
    /// ``0x3BA | MRW | Physical memory protection address register.``
    Pmpaddr10,
    /// ``0x3BB | MRW | Physical memory protection address register.``
    Pmpaddr11,
    /// ``0x3BC | MRW | Physical memory protection address register.``
    Pmpaddr12,
    /// ``0x3BD | MRW | Physical memory protection address register.``
    Pmpaddr13,
    /// ``0x3BE | MRW | Physical memory protection address register.``
    Pmpaddr14,
    /// ``0x3BF | MRW | Physical memory protection address register.``
    Pmpaddr15,
    /// ``0x3C0 | MRW | Physical memory protection address register.``
    Pmpaddr16,
    /// ``0x3C1 | MRW | Physical memory protection address register.``
    Pmpaddr17,
    /// ``0x3C2 | MRW | Physical memory protection address register.``
    Pmpaddr18,
    /// ``0x3C3 | MRW | Physical memory protection address register.``
    Pmpaddr19,
    /// ``0x3C4 | MRW | Physical memory protection address register.``
    Pmpaddr20,
    /// ``0x3C5 | MRW | Physical memory protection address register.``
    Pmpaddr21,
    /// ``0x3C6 | MRW | Physical memory protection address register.``
    Pmpaddr22,
    /// ``0x3C7 | MRW | Physical memory protection address register.``
    Pmpaddr23,
    /// ``0x3C8 | MRW | Physical memory protection address register.``
    Pmpaddr24,
    /// ``0x3C9 | MRW | Physical memory protection address register.``
    Pmpaddr25,
    /// ``0x3CA | MRW | Physical memory protection address register.``
    Pmpaddr26,
    /// ``0x3CB | MRW | Physical memory protection address register.``
    Pmpaddr27,
    /// ``0x3CC | MRW | Physical memory protection address register.``
    Pmpaddr28,
    /// ``0x3CD | MRW | Physical memory protection address register.``
    Pmpaddr29,
    /// ``0x3CE | MRW | Physical memory protection address register.``
    Pmpaddr30,
    /// ``0x3CF | MRW | Physical memory protection address register.``
    Pmpaddr31,
    /// ``0x3D0 | MRW | Physical memory protection address register.``
    Pmpaddr32,
    /// ``0x3D1 | MRW | Physical memory protection address register.``
    Pmpaddr33,
    /// ``0x3D2 | MRW | Physical memory protection address register.``
    Pmpaddr34,
    /// ``0x3D3 | MRW | Physical memory protection address register.``
    Pmpaddr35,
    /// ``0x3D4 | MRW | Physical memory protection address register.``
    Pmpaddr36,
    /// ``0x3D5 | MRW | Physical memory protection address register.``
    Pmpaddr37,
    /// ``0x3D6 | MRW | Physical memory protection address register.``
    Pmpaddr38,
    /// ``0x3D7 | MRW | Physical memory protection address register.``
    Pmpaddr39,
    /// ``0x3D8 | MRW | Physical memory protection address register.``
    Pmpaddr40,
    /// ``0x3D9 | MRW | Physical memory protection address register.``
    Pmpaddr41,
    /// ``0x3DA | MRW | Physical memory protection address register.``
    Pmpaddr42,
    /// ``0x3DB | MRW | Physical memory protection address register.``
    Pmpaddr43,
    /// ``0x3DC | MRW | Physical memory protection address register.``
    Pmpaddr44,
    /// ``0x3DD | MRW | Physical memory protection address register.``
    Pmpaddr45,
    /// ``0x3DE | MRW | Physical memory protection address register.``
    Pmpaddr46,
    /// ``0x3DF | MRW | Physical memory protection address register.``
    Pmpaddr47,
    /// ``0x3E0 | MRW | Physical memory protection address register.``
    Pmpaddr48,
    /// ``0x3E1 | MRW | Physical memory protection address register.``
    Pmpaddr49,
    /// ``0x3E2 | MRW | Physical memory protection address register.``
    Pmpaddr50,
    /// ``0x3E3 | MRW | Physical memory protection address register.``
    Pmpaddr51,
    /// ``0x3E4 | MRW | Physical memory protection address register.``
    Pmpaddr52,
    /// ``0x3E5 | MRW | Physical memory protection address register.``
    Pmpaddr53,
    /// ``0x3E6 | MRW | Physical memory protection address register.``
    Pmpaddr54,
    /// ``0x3E7 | MRW | Physical memory protection address register.``
    Pmpaddr55,
    /// ``0x3E8 | MRW | Physical memory protection address register.``
    Pmpaddr56,
    /// ``0x3E9 | MRW | Physical memory protection address register.``
    Pmpaddr57,
    /// ``0x3EA | MRW | Physical memory protection address register.``
    Pmpaddr58,
    /// ``0x3EB | MRW | Physical memory protection address register.``
    Pmpaddr59,
    /// ``0x3EC | MRW | Physical memory protection address register.``
    Pmpaddr60,
    /// ``0x3ED | MRW | Physical memory protection address register.``
    Pmpaddr61,
    /// ``0x3EE | MRW | Physical memory protection address register.``
    Pmpaddr62,
    /// ``0x3EF | MRW | Physical memory protection address register.``
    Pmpaddr63,

    // Machine Counters/Timers
    /// ``0xB00 | MRW | Machine cycle counter.``
    Mcycle,
    /// ``0xB02 | MRW | Machine instructions-retired counter.``
    Minstret,
    /// ``0xB03 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter3,
    /// ``0xB04 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter4,
    /// ``0xB05 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter5,
    /// ``0xB06 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter6,
    /// ``0xB07 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter7,
    /// ``0xB08 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter8,
    /// ``0xB09 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter9,
    /// ``0xB0A | MRW | Machine performance-monitoring counter.``
    Mhpmcounter10,
    /// ``0xB0B | MRW | Machine performance-monitoring counter.``
    Mhpmcounter11,
    /// ``0xB0C | MRW | Machine performance-monitoring counter.``
    Mhpmcounter12,
    /// ``0xB0D | MRW | Machine performance-monitoring counter.``
    Mhpmcounter13,
    /// ``0xB0E | MRW | Machine performance-monitoring counter.``
    Mhpmcounter14,
    /// ``0xB0F | MRW | Machine performance-monitoring counter.``
    Mhpmcounter15,
    /// ``0xB10 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter16,
    /// ``0xB11 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter17,
    /// ``0xB12 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter18,
    /// ``0xB13 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter19,
    /// ``0xB14 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter20,
    /// ``0xB15 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter21,
    /// ``0xB16 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter22,
    /// ``0xB17 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter23,
    /// ``0xB18 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter24,
    /// ``0xB19 | MRW | Machine performance-monitoring counter.``
    Mhpmcounter25,
    /// ``0xB1A | MRW | Machine performance-monitoring counter.``
    Mhpmcounter26,
    /// ``0xB1B | MRW | Machine performance-monitoring counter.``
    Mhpmcounter27,
    /// ``0xB1C | MRW | Machine performance-monitoring counter.``
    Mhpmcounter28,
    /// ``0xB1D | MRW | Machine performance-monitoring counter.``
    Mhpmcounter29,
    /// ``0xB1E | MRW | Machine performance-monitoring counter.``
    Mhpmcounter30,
    /// ``0xB1F | MRW | Machine performance-monitoring counter.``
    Mhpmcounter31,

    /// ``0xB80 | MRW | Upper 32 bits of `Mcycle`, RV32 only.``
    Mcycleh,
    /// ``0xB82 | MRW | Upper 32 bits of `Minstret`, RV32 only.``
    Minstreth,
    /// ``0xB83 | MRW | Upper 32 bits of `Mhpmcounter3`, RV32 only.``
    Mhpmcounter3h,
    /// ``0xB84 | MRW | Upper 32 bits of `Mhpmcounter4`, RV32 only.``
    Mhpmcounter4h,
    /// ``0xB85 | MRW | Upper 32 bits of `Mhpmcounter5`, RV32 only.``
    Mhpmcounter5h,
    /// ``0xB86 | MRW | Upper 32 bits of `Mhpmcounter6`, RV32 only.``
    Mhpmcounter6h,
    /// ``0xB87 | MRW | Upper 32 bits of `Mhpmcounter7`, RV32 only.``
    Mhpmcounter7h,
    /// ``0xB88 | MRW | Upper 32 bits of `Mhpmcounter8`, RV32 only.``
    Mhpmcounter8h,
    /// ``0xB89 | MRW | Upper 32 bits of `Mhpmcounter9`, RV32 only.``
    Mhpmcounter9h,
    /// ``0xB8A | MRW | Upper 32 bits of `Mhpmcounter10`, RV32 only.``
    Mhpmcounter10h,
    /// ``0xB8B | MRW | Upper 32 bits of `Mhpmcounter11`, RV32 only.``
    Mhpmcounter11h,
    /// ``0xB8C | MRW | Upper 32 bits of `Mhpmcounter12`, RV32 only.``
    Mhpmcounter12h,
    /// ``0xB8D | MRW | Upper 32 bits of `Mhpmcounter13`, RV32 only.``
    Mhpmcounter13h,
    /// ``0xB8E | MRW | Upper 32 bits of `Mhpmcounter14`, RV32 only.``
    Mhpmcounter14h,
    /// ``0xB8F | MRW | Upper 32 bits of `Mhpmcounter15`, RV32 only.``
    Mhpmcounter15h,
    /// ``0xB90 | MRW | Upper 32 bits of `Mhpmcounter16`, RV32 only.``
    Mhpmcounter16h,
    /// ``0xB91 | MRW | Upper 32 bits of `Mhpmcounter17`, RV32 only.``
    Mhpmcounter17h,
    /// ``0xB92 | MRW | Upper 32 bits of `Mhpmcounter18`, RV32 only.``
    Mhpmcounter18h,
    /// ``0xB93 | MRW | Upper 32 bits of `Mhpmcounter19`, RV32 only.``
    Mhpmcounter19h,
    /// ``0xB94 | MRW | Upper 32 bits of `Mhpmcounter20`, RV32 only.``
    Mhpmcounter20h,
    /// ``0xB95 | MRW | Upper 32 bits of `Mhpmcounter21`, RV32 only.``
    Mhpmcounter21h,
    /// ``0xB96 | MRW | Upper 32 bits of `Mhpmcounter22`, RV32 only.``
    Mhpmcounter22h,
    /// ``0xB97 | MRW | Upper 32 bits of `Mhpmcounter23`, RV32 only.``
    Mhpmcounter23h,
    /// ``0xB98 | MRW | Upper 32 bits of `Mhpmcounter24`, RV32 only.``
    Mhpmcounter24h,
    /// ``0xB99 | MRW | Upper 32 bits of `Mhpmcounter25`, RV32 only.``
    Mhpmcounter25h,
    /// ``0xB9A | MRW | Upper 32 bits of `Mhpmcounter26`, RV32 only.``
    Mhpmcounter26h,
    /// ``0xB9B | MRW | Upper 32 bits of `Mhpmcounter27`, RV32 only.``
    Mhpmcounter27h,
    /// ``0xB9C | MRW | Upper 32 bits of `Mhpmcounter28`, RV32 only.``
    Mhpmcounter28h,
    /// ``0xB9D | MRW | Upper 32 bits of `Mhpmcounter29`, RV32 only.``
    Mhpmcounter29h,
    /// ``0xB9E | MRW | Upper 32 bits of `Mhpmcounter30`, RV32 only.``
    Mhpmcounter30h,
    /// ``0xB9F | MRW | Upper 32 bits of `Mhpmcounter31`, RV32 only.``
    Mhpmcounter31h,

    // Machine Counter Setup
    /// ``0x320 | MRW | Machine counter-inhibit register.``
    Mcountinhibit,
    /// ``0x323 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent3,
    /// ``0x324 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent4,
    /// ``0x325 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent5,
    /// ``0x326 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent6,
    /// ``0x327 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent7,
    /// ``0x328 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent8,
    /// ``0x329 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent9,
    /// ``0x32A | MRW | Machine performance-monitoring event selector.``
    Mhpmevent10,
    /// ``0x32B | MRW | Machine performance-monitoring event selector.``
    Mhpmevent11,
    /// ``0x32C | MRW | Machine performance-monitoring event selector.``
    Mhpmevent12,
    /// ``0x32D | MRW | Machine performance-monitoring event selector.``
    Mhpmevent13,
    /// ``0x32E | MRW | Machine performance-monitoring event selector.``
    Mhpmevent14,
    /// ``0x32F | MRW | Machine performance-monitoring event selector.``
    Mhpmevent15,
    /// ``0x330 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent16,
    /// ``0x331 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent17,
    /// ``0x332 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent18,
    /// ``0x333 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent19,
    /// ``0x334 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent20,
    /// ``0x335 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent21,
    /// ``0x336 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent22,
    /// ``0x337 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent23,
    /// ``0x338 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent24,
    /// ``0x339 | MRW | Machine performance-monitoring event selector.``
    Mhpmevent25,
    /// ``0x33A | MRW | Machine performance-monitoring event selector.``
    Mhpmevent26,
    /// ``0x33B | MRW | Machine performance-monitoring event selector.``
    Mhpmevent27,
    /// ``0x33C | MRW | Machine performance-monitoring event selector.``
    Mhpmevent28,
    /// ``0x33D | MRW | Machine performance-monitoring event selector.``
    Mhpmevent29,
    /// ``0x33E | MRW | Machine performance-monitoring event selector.``
    Mhpmevent30,
    /// ``0x33F | MRW | Machine performance-monitoring event selector.``
    Mhpmevent31,

    // Debug/Trace Registers (shared with Debug Mode)
    /// ``0x7A0 | MRW | Debug/Trace trigger register select.``
    Tselect,
    /// ``0x7A1 | MRW | First Debug/Trace trigger data register.``
    Tdata1,
    /// ``0x7A2 | MRW | Second Debug/Trace trigger data register.``
    Tdata2,
    /// ``0x7A3 | MRW | Third Debug/Trace trigger data register.``
    Tdata3,
    /// ``0x7A8 | MRW | Machine-mode context register.``
    Mcontext,

    // Debug Mode Registers
    /// ``0x7B0 | DRW | Debug control and status register.``
    Dcsr,
    /// ``0x7B1 | DRW | Debug PC.``
    Dpc,
    /// ``0x7B2 | DRW | Debug scratch register 0.``
    Dscratch0,
    /// ``0x7B3 | DRW | Debug scratch register 1.``
    Dscratch1,
}

const CSR_FILE_SIZE: usize = Csr::Dscratch1 as usize + 1;

impl Csr {
    pub fn checked_from_u32(r: u32) -> Option<Self> {
        use Csr::*;
        let csr = match r {
            0x001 => Fflags,
            0x002 => Frm,
            0x003 => Fcsr,
            0xC00 => Cycle,
            0xC01 => Time,
            0xC02 => Instret,
            0xC03 => Hpmcounter3,
            0xC04 => Hpmcounter4,
            0xC05 => Hpmcounter5,
            0xC06 => Hpmcounter6,
            0xC07 => Hpmcounter7,
            0xC08 => Hpmcounter8,
            0xC09 => Hpmcounter9,
            0xC0A => Hpmcounter10,
            0xC0B => Hpmcounter11,
            0xC0C => Hpmcounter12,
            0xC0D => Hpmcounter13,
            0xC0E => Hpmcounter14,
            0xC0F => Hpmcounter15,
            0xC10 => Hpmcounter16,
            0xC11 => Hpmcounter17,
            0xC12 => Hpmcounter18,
            0xC13 => Hpmcounter19,
            0xC14 => Hpmcounter20,
            0xC15 => Hpmcounter21,
            0xC16 => Hpmcounter22,
            0xC17 => Hpmcounter23,
            0xC18 => Hpmcounter24,
            0xC19 => Hpmcounter25,
            0xC1A => Hpmcounter26,
            0xC1B => Hpmcounter27,
            0xC1C => Hpmcounter28,
            0xC1D => Hpmcounter29,
            0xC1E => Hpmcounter30,
            0xC1F => Hpmcounter31,
            0xC80 => Cycleh,
            0xC81 => Timeh,
            0xC82 => Instreth,
            0xC83 => Hpmcounter3h,
            0xC84 => Hpmcounter4h,
            0xC85 => Hpmcounter5h,
            0xC86 => Hpmcounter6h,
            0xC87 => Hpmcounter7h,
            0xC88 => Hpmcounter8h,
            0xC89 => Hpmcounter9h,
            0xC8A => Hpmcounter10h,
            0xC8B => Hpmcounter11h,
            0xC8C => Hpmcounter12h,
            0xC8D => Hpmcounter13h,
            0xC8E => Hpmcounter14h,
            0xC8F => Hpmcounter15h,
            0xC90 => Hpmcounter16h,
            0xC91 => Hpmcounter17h,
            0xC92 => Hpmcounter18h,
            0xC93 => Hpmcounter19h,
            0xC94 => Hpmcounter20h,
            0xC95 => Hpmcounter21h,
            0xC96 => Hpmcounter22h,
            0xC97 => Hpmcounter23h,
            0xC98 => Hpmcounter24h,
            0xC99 => Hpmcounter25h,
            0xC9A => Hpmcounter26h,
            0xC9B => Hpmcounter27h,
            0xC9C => Hpmcounter28h,
            0xC9D => Hpmcounter29h,
            0xC9E => Hpmcounter30h,
            0xC9F => Hpmcounter31h,
            0x100 => Sstatus,
            0x104 => Ssie,
            0x105 => Stvec,
            0x106 => Scounteren,
            0x10A => Senvcfg,
            0x140 => Sscratch,
            0x141 => Sepc,
            0x142 => Scause,
            0x143 => Stval,
            0x144 => Sip,
            0x180 => Satp,
            0x5A8 => Scontext,
            0x600 => Hstatus,
            0x602 => Hedeleg,
            0x603 => Hideleg,
            0x604 => Hie,
            0x606 => Hcounteren,
            0x607 => Hgeie,
            0x643 => Htval,
            0x644 => Hip,
            0x645 => Hvip,
            0x64A => Htinst,
            0xE12 => Hgeip,
            0x60A => Henvcfg,
            0x61A => Henvcfgh,
            0x6A8 => Hcontext,
            0x605 => Htimedelta,
            0x615 => Htimedeltah,
            0x200 => Vsstatus,
            0x204 => Vsie,
            0x205 => Vstvec,
            0x240 => Vsscratch,
            0x241 => Vsepc,
            0x242 => Vscause,
            0x243 => Vstval,
            0x244 => Vsip,
            0x280 => Vsatp,
            0xF11 => Mvendorid,
            0xF12 => Marchid,
            0xF13 => Mimpid,
            0xF14 => Mhartid,
            0xF15 => Mconfigptr,
            0x300 => Mstatus,
            0x301 => Misa,
            0x302 => Medeleg,
            0x303 => Mideleg,
            0x304 => Mie,
            0x305 => Mtvec,
            0x306 => Mcounteren,
            0x310 => Mstatush,
            0x340 => Mscratch,
            0x341 => Mepc,
            0x342 => Mcause,
            0x343 => Mtval,
            0x344 => Mip,
            0x34A => Mtinst,
            0x34B => Mtval2,
            0x30A => Menvcfg,
            0x31A => Menvcfgh,
            0x747 => Mseccfg,
            0x757 => Mseccfgh,
            0x3A0 => Pmpcfg0,
            0x3A1 => Pmpcfg1,
            0x3A2 => Pmpcfg2,
            0x3A3 => Pmpcfg3,
            0x3A4 => Pmpcfg4,
            0x3A5 => Pmpcfg5,
            0x3A6 => Pmpcfg6,
            0x3A7 => Pmpcfg7,
            0x3A8 => Pmpcfg08,
            0x3A9 => Pmpcfg09,
            0x3AA => Pmpcfg10,
            0x3AB => Pmpcfg11,
            0x3AC => Pmpcfg12,
            0x3AD => Pmpcfg13,
            0x3AE => Pmpcfg14,
            0x3AF => Pmpcfg15,
            0x3B0 => Pmpaddr0,
            0x3B1 => Pmpaddr1,
            0x3B2 => Pmpaddr2,
            0x3B3 => Pmpaddr3,
            0x3B4 => Pmpaddr4,
            0x3B5 => Pmpaddr5,
            0x3B6 => Pmpaddr6,
            0x3B7 => Pmpaddr7,
            0x3B8 => Pmpaddr8,
            0x3B9 => Pmpaddr9,
            0x3BA => Pmpaddr10,
            0x3BB => Pmpaddr11,
            0x3BC => Pmpaddr12,
            0x3BD => Pmpaddr13,
            0x3BE => Pmpaddr14,
            0x3BF => Pmpaddr15,
            0x3C0 => Pmpaddr16,
            0x3C1 => Pmpaddr17,
            0x3C2 => Pmpaddr18,
            0x3C3 => Pmpaddr19,
            0x3C4 => Pmpaddr20,
            0x3C5 => Pmpaddr21,
            0x3C6 => Pmpaddr22,
            0x3C7 => Pmpaddr23,
            0x3C8 => Pmpaddr24,
            0x3C9 => Pmpaddr25,
            0x3CA => Pmpaddr26,
            0x3CB => Pmpaddr27,
            0x3CC => Pmpaddr28,
            0x3CD => Pmpaddr29,
            0x3CE => Pmpaddr30,
            0x3CF => Pmpaddr31,
            0x3D0 => Pmpaddr32,
            0x3D1 => Pmpaddr33,
            0x3D2 => Pmpaddr34,
            0x3D3 => Pmpaddr35,
            0x3D4 => Pmpaddr36,
            0x3D5 => Pmpaddr37,
            0x3D6 => Pmpaddr38,
            0x3D7 => Pmpaddr39,
            0x3D8 => Pmpaddr40,
            0x3D9 => Pmpaddr41,
            0x3DA => Pmpaddr42,
            0x3DB => Pmpaddr43,
            0x3DC => Pmpaddr44,
            0x3DD => Pmpaddr45,
            0x3DE => Pmpaddr46,
            0x3DF => Pmpaddr47,
            0x3E0 => Pmpaddr48,
            0x3E1 => Pmpaddr49,
            0x3E2 => Pmpaddr50,
            0x3E3 => Pmpaddr51,
            0x3E4 => Pmpaddr52,
            0x3E5 => Pmpaddr53,
            0x3E6 => Pmpaddr54,
            0x3E7 => Pmpaddr55,
            0x3E8 => Pmpaddr56,
            0x3E9 => Pmpaddr57,
            0x3EA => Pmpaddr58,
            0x3EB => Pmpaddr59,
            0x3EC => Pmpaddr60,
            0x3ED => Pmpaddr61,
            0x3EE => Pmpaddr62,
            0x3EF => Pmpaddr63,
            0xB00 => Mcycle,
            0xB02 => Minstret,
            0xB03 => Mhpmcounter3,
            0xB04 => Mhpmcounter4,
            0xB05 => Mhpmcounter5,
            0xB06 => Mhpmcounter6,
            0xB07 => Mhpmcounter7,
            0xB08 => Mhpmcounter8,
            0xB09 => Mhpmcounter9,
            0xB0A => Mhpmcounter10,
            0xB0B => Mhpmcounter11,
            0xB0C => Mhpmcounter12,
            0xB0D => Mhpmcounter13,
            0xB0E => Mhpmcounter14,
            0xB0F => Mhpmcounter15,
            0xB10 => Mhpmcounter16,
            0xB11 => Mhpmcounter17,
            0xB12 => Mhpmcounter18,
            0xB13 => Mhpmcounter19,
            0xB14 => Mhpmcounter20,
            0xB15 => Mhpmcounter21,
            0xB16 => Mhpmcounter22,
            0xB17 => Mhpmcounter23,
            0xB18 => Mhpmcounter24,
            0xB19 => Mhpmcounter25,
            0xB1A => Mhpmcounter26,
            0xB1B => Mhpmcounter27,
            0xB1C => Mhpmcounter28,
            0xB1D => Mhpmcounter29,
            0xB1E => Mhpmcounter30,
            0xB1F => Mhpmcounter31,
            0xB80 => Mcycleh,
            0xB82 => Minstreth,
            0xB83 => Mhpmcounter3h,
            0xB84 => Mhpmcounter4h,
            0xB85 => Mhpmcounter5h,
            0xB86 => Mhpmcounter6h,
            0xB87 => Mhpmcounter7h,
            0xB88 => Mhpmcounter8h,
            0xB89 => Mhpmcounter9h,
            0xB8A => Mhpmcounter10h,
            0xB8B => Mhpmcounter11h,
            0xB8C => Mhpmcounter12h,
            0xB8D => Mhpmcounter13h,
            0xB8E => Mhpmcounter14h,
            0xB8F => Mhpmcounter15h,
            0xB90 => Mhpmcounter16h,
            0xB91 => Mhpmcounter17h,
            0xB92 => Mhpmcounter18h,
            0xB93 => Mhpmcounter19h,
            0xB94 => Mhpmcounter20h,
            0xB95 => Mhpmcounter21h,
            0xB96 => Mhpmcounter22h,
            0xB97 => Mhpmcounter23h,
            0xB98 => Mhpmcounter24h,
            0xB99 => Mhpmcounter25h,
            0xB9A => Mhpmcounter26h,
            0xB9B => Mhpmcounter27h,
            0xB9C => Mhpmcounter28h,
            0xB9D => Mhpmcounter29h,
            0xB9E => Mhpmcounter30h,
            0xB9F => Mhpmcounter31h,
            0x320 => Mcountinhibit,
            0x323 => Mhpmevent3,
            0x324 => Mhpmevent4,
            0x325 => Mhpmevent5,
            0x326 => Mhpmevent6,
            0x327 => Mhpmevent7,
            0x328 => Mhpmevent8,
            0x329 => Mhpmevent9,
            0x32A => Mhpmevent10,
            0x32B => Mhpmevent11,
            0x32C => Mhpmevent12,
            0x32D => Mhpmevent13,
            0x32E => Mhpmevent14,
            0x32F => Mhpmevent15,
            0x330 => Mhpmevent16,
            0x331 => Mhpmevent17,
            0x332 => Mhpmevent18,
            0x333 => Mhpmevent19,
            0x334 => Mhpmevent20,
            0x335 => Mhpmevent21,
            0x336 => Mhpmevent22,
            0x337 => Mhpmevent23,
            0x338 => Mhpmevent24,
            0x339 => Mhpmevent25,
            0x33A => Mhpmevent26,
            0x33B => Mhpmevent27,
            0x33C => Mhpmevent28,
            0x33D => Mhpmevent29,
            0x33E => Mhpmevent30,
            0x33F => Mhpmevent31,
            0x7A0 => Tselect,
            0x7A1 => Tdata1,
            0x7A2 => Tdata2,
            0x7A3 => Tdata3,
            0x7A8 => Mcontext,
            0x7B0 => Dcsr,
            0x7B1 => Dpc,
            0x7B2 => Dscratch0,
            0x7B3 => Dscratch1,
            _ => None?,
        };

        Some(csr)
    }

    pub fn decode_raw32(raw32: u32) -> Option<Self> {
        Self::checked_from_u32(raw32 >> 20)
    }
}

pub struct CsrFile {
    reg: [u32; CSR_FILE_SIZE],
}

impl Default for CsrFile {
    fn default() -> Self {
        Self::new()
    }
}

impl CsrFile {
    pub fn new() -> Self {
        Self {
            reg: [0; CSR_FILE_SIZE],
        }
    }
}
