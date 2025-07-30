use crate::{Cpu6502, Operand};

pub mod absolute;
pub mod immediate;
pub mod implicit;

pub trait Addressing<O: Operand> {
    fn resolve(cpu: &mut Cpu6502) -> O;
}
