use crate::{operand::Imp, Bus, Cpu6502, Instruction};

pub struct Jam;
impl<B: Bus> Instruction<B, Imp> for Jam {
    const NAME: &'static str = "jam";
    const ILLEGAL: bool = true;

    fn exec(_cpu: &mut Cpu6502<B>, _: Imp) {}
}
