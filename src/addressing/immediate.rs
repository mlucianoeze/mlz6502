use crate::{addressing::Addressing, operand::Val, Cpu6502};

pub struct Immediate;
impl Addressing<Val> for Immediate {
    fn resolve(cpu: &mut Cpu6502) -> Val {
        Val(cpu.fetch())
    }
}
