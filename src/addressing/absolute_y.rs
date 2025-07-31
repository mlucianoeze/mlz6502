use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::{Addr, Val};

pub struct AbsoluteY;

impl Addressing<Addr> for AbsoluteY {
    fn resolve(cpu: &mut Cpu6502) -> Addr {
        Addr(cpu.fetch16().wrapping_add(cpu.y as u16))
    }
}

impl Addressing<Val> for AbsoluteY {
    fn resolve(cpu: &mut Cpu6502) -> Val {
        let Addr(addr) = Self::resolve(cpu);
        Val(cpu.bus.read(addr))
    }
}
