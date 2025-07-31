use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::{Addr, Val};

pub struct AbsoluteX;

impl Addressing<Addr> for AbsoluteX {
    fn resolve(cpu: &mut Cpu6502) -> Addr {
        Addr(cpu.fetch16().wrapping_add(cpu.x as u16))
    }
}

impl Addressing<Val> for AbsoluteX {
    fn resolve(cpu: &mut Cpu6502) -> Val {
        let Addr(addr) = Self::resolve(cpu);
        Val(cpu.bus.read(addr))
    }
}
