use crate::cpu::Cpu6502;
use crate::instruction::Instruction;
use crate::operand::Val;

pub struct Cpx;
impl Instruction<Val> for Cpx {
    const NAME: &'static str = "cpx";

    fn exec(cpu: &mut Cpu6502, Val(v): Val) {
        let x = cpu.x;
        cpu.set_carry(x >= v);
        cpu.set_zero(x == v);
        cpu.set_negative(x.wrapping_sub(v));
    }
}
