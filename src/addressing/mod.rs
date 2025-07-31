use crate::cpu::Cpu6502;
use crate::operand::Operand;

pub mod absolute;
pub mod immediate;
pub mod implicit;

pub trait Addressing<O: Operand> {
    fn resolve(cpu: &mut Cpu6502) -> O;
}
