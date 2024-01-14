// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright (C) 2024 mumblingdrunkard

pub mod exec;

use crate::{
    freg::{FRegFile, FRegType},
    reg::{RegFile, RegType},
};

pub struct Hart<
    const ID: usize,
    I: RegType,
    const M: bool,
    const A: bool,
    F: FRegType,
    const ZIFENCEI: bool,
    const C: bool,
> {
    pc: I,
    reg: RegFile<I>,
    freg: FRegFile<F>,
}
