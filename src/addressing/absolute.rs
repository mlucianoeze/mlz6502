use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::{Addr, Val};

pub struct Absolute;
impl Absolute {
    #[inline]
    fn eff_addr(cpu: &mut Cpu6502) -> u16 {
        cpu.fetch16()
    }
}

impl Addressing<Addr> for Absolute {
    fn resolve(cpu: &mut Cpu6502) -> Addr {
        Addr(Self::eff_addr(cpu))
    }
}

impl Addressing<Val> for Absolute {
    fn resolve(cpu: &mut Cpu6502) -> Val {
        let addr = Self::eff_addr(cpu);
        Val(cpu.bus.read(addr))
    }
}
