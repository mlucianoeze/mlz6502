use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::{Addr, Val};

pub struct ZeroPageY;

impl Addressing<Addr> for ZeroPageY {
    fn resolve(cpu: &mut Cpu6502) -> Addr {
        let offset = cpu.fetch().wrapping_add(cpu.y);
        Addr(offset as u16)
    }
}

impl Addressing<Val> for ZeroPageY {
    fn resolve(cpu: &mut Cpu6502) -> Val {
        let Addr(addr) = Self::resolve(cpu);
        Val(cpu.bus.read(addr))
    }
}
