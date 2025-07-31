use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::{Addr, Val};

pub struct IndirectX;

impl Addressing<Addr> for IndirectX {
    fn resolve(cpu: &mut Cpu6502) -> Addr {
        let offset = cpu.fetch().wrapping_add(cpu.x); // NB: this is a bug in original hardware
        let pointer = offset as u16;
        Addr(cpu.bus.read16(pointer))
    }
}
impl Addressing<Val> for IndirectX {
    fn resolve(cpu: &mut Cpu6502) -> Val {
        let Addr(addr) = Self::resolve(cpu);
        Val(cpu.bus.read(addr))
    }
}
