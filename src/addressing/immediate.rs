use crate::{addressing::Addressing, operand::Val, Bus, Cpu6502};

pub struct Immediate;
impl<B: Bus> Addressing<B, Val> for Immediate {
    fn resolve(cpu: &mut Cpu6502<B>) -> Val {
        Val(cpu.fetch())
    }
}
