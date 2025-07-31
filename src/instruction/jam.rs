use crate::cpu::Cpu6502;
use crate::instruction::Instruction;
use crate::operand::Imp;

pub struct Jam;
impl Instruction<Imp> for Jam {
    const NAME: &'static str = "jam";
    const ILLEGAL: bool = true;

    fn exec(_cpu: &mut Cpu6502, _: Imp) {}
}
