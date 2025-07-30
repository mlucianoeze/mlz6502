use crate::{operand::Val, Bus, Cpu6502, Instruction};

pub struct Cpx;
impl<B: Bus> Instruction<B, Val> for Cpx {
    const NAME: &'static str = "cpx";

    fn exec(cpu: &mut Cpu6502<B>, Val(v): Val) {
        let x = cpu.x;
        cpu.set_carry(x >= v);
        cpu.set_zero(x == v);
        cpu.set_negative(x.wrapping_sub(v));
    }
}
