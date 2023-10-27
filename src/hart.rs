pub mod exec;

use crate::{inst::Instruction, reg::RegFile};

pub struct Hart<const ID: usize> {
    pc: u32,
    reg: RegFile,
}

impl<const ID: usize> Hart<ID> {
    pub fn exec(&mut self, inst: Instruction) -> Result<(), exec::Error> {
        todo!()
    }
}
