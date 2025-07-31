use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::Val;

pub struct Immediate;
impl Addressing<Val> for Immediate {
    fn resolve(cpu: &mut Cpu6502) -> Val {
        Val(cpu.fetch())
    }
}
