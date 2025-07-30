use crate::{operand::Imp, Cpu6502, Instruction};

pub struct Jam;
impl Instruction<Imp> for Jam {
    const NAME: &'static str = "jam";
    const ILLEGAL: bool = true;

    fn exec(_cpu: &mut Cpu6502, _: Imp) {}
}
