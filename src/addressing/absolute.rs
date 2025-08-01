use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::{Addr, Val};

pub struct Absolute;

impl Addressing<Addr> for Absolute {
    fn resolve(cpu: &mut Cpu6502) -> Addr {
        Addr(cpu.fetch16())
    }
}

impl Addressing<Val> for Absolute {
    fn resolve(cpu: &mut Cpu6502) -> Val {
        let Addr(addr) = Self::resolve(cpu);
        Val(cpu.bus.read(addr))
    }
}
