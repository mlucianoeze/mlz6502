use crate::{operand::Addr, Cpu6502, Instruction};

pub struct Sta;
impl Instruction<Addr> for Sta {
    const NAME: &'static str = "sta";

    fn exec(cpu: &mut Cpu6502, Addr(addr): Addr) {
        cpu.bus.write(addr, cpu.a);
    }
}
