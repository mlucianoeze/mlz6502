use crate::{
    addressing::Addressing,
    operand::{Addr, Val},
    Bus, Cpu6502,
};

pub struct Absolute;
impl Absolute {
    #[inline]
    fn eff_addr<B: Bus>(cpu: &mut Cpu6502<B>) -> u16 {
        cpu.fetch16()
    }
}

impl<B: Bus> Addressing<B, Addr> for Absolute {
    fn resolve(cpu: &mut Cpu6502<B>) -> Addr {
        Addr(Self::eff_addr(cpu))
    }
}

impl<B: Bus> Addressing<B, Val> for Absolute {
    fn resolve(cpu: &mut Cpu6502<B>) -> Val {
        let addr = Self::eff_addr(cpu);
        Val(cpu.bus.read(addr))
    }
}
