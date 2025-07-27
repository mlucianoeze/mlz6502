use crate::{operand::Addr, Bus, Cpu6502, Instruction};

pub struct Sta;
impl<B: Bus> Instruction<B, Addr> for Sta {
    fn exec(cpu: &mut Cpu6502<B>, Addr(addr): Addr) {
        cpu.bus.write(addr, cpu.a);
    }
}
