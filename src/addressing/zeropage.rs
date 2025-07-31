use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::{Addr, Val};

pub struct ZeroPage;

impl Addressing<Addr> for ZeroPage {
    fn resolve(cpu: &mut Cpu6502) -> Addr {
        Addr(cpu.fetch() as u16)
    }
}

impl Addressing<Val> for ZeroPage {
    fn resolve(cpu: &mut Cpu6502) -> Val {
        let Addr(addr) = Self::resolve(cpu);
        Val(cpu.bus.read(addr))
    }
}
