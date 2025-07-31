use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::{Addr, Val};

pub struct IndirectY;

impl Addressing<Addr> for IndirectY {
    fn resolve(cpu: &mut Cpu6502) -> Addr {
        let offset = cpu.fetch();
        let pointer = offset as u16;
        let addr = cpu.bus.read16(pointer);
        Addr(addr.wrapping_add(cpu.y as u16))
    }
}
impl Addressing<Val> for IndirectY {
    fn resolve(cpu: &mut Cpu6502) -> Val {
        let Addr(addr) = Self::resolve(cpu);
        Val(cpu.bus.read(addr))
    }
}
