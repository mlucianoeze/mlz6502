use crate::{operand::Val, Bus, Cpu6502, Instruction};

pub struct Lda;
impl<B: Bus> Instruction<B, Val> for Lda {
    fn exec(cpu: &mut Cpu6502<B>, Val(v): Val) {
        cpu.a = v;
        cpu.set_zero(cpu.a == 0);
        cpu.set_negative(cpu.a);
    }
}
