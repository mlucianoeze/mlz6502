use crate::{operand::Val, Cpu6502, Instruction};

pub struct Lda;
impl Instruction<Val> for Lda {
    const NAME: &'static str = "lda";

    fn exec(cpu: &mut Cpu6502, Val(v): Val) {
        cpu.a = v;
        cpu.set_zero(cpu.a == 0);
        cpu.set_negative(cpu.a);
    }
}
