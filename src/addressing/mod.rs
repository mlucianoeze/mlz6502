use crate::{Bus, Cpu6502, Operand};

pub mod absolute;
pub mod immediate;
pub mod implicit;

pub trait Addressing<B: Bus, O: Operand> {
    fn resolve(cpu: &mut Cpu6502<B>) -> O;
}
