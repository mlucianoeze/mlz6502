use crate::{addressing::Addressing, operand::Imp, Bus, Cpu6502};

pub struct Implicit;
impl<B: Bus> Addressing<B, Imp> for Implicit {
    fn resolve(_cpu: &mut Cpu6502<B>) -> Imp {
        Imp
    }
}
