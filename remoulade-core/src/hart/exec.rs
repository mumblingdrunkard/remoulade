// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright (C) 2024 mumblingdrunkard

pub enum Error {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreOrAmoAddressMisaligned = 6,
    StoreOrAmoAccessFault = 7,
    EcallFromUOrVUMode = 8,
    EcallFromHSMode = 9,
    EcallFromVSMode = 10,
    EcallFromMMode = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StoreOrAmoPageFault = 15,
    InstructionGuestPageFault = 20,
    LoadGuestPageFault = 21,

    /// Raised when virtualisation is enabled and software attempts to access a VS CSR directly
    ///
    /// > When V=1, an attempt to read or write a VS CSR directly by its own separate CSR address causes a virtual instruction exception.
    /// >
    /// > --- RISC-V privileged specification, p. 100
    VirtualInstruction = 22,

    StoreOrAmoGuestPageFault = 23,
}
