use crate::cpu::Cpu6502;
use crate::operand::Operand;

pub mod absolute;
pub mod absolute_x;
pub mod absolute_y;
pub mod accumulator;
pub mod immediate;
pub mod implicit;
pub mod indirect;
pub mod indirect_x;
pub mod indirect_y;
pub mod relative;
pub mod zeropage;
pub mod zeropage_x;
pub mod zeropage_y;

pub trait Addressing<O: Operand> {
    fn resolve(cpu: &mut Cpu6502) -> O;
}
