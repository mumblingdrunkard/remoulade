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
