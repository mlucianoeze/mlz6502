use crate::{addressing::Addressing, operand::Imp, Cpu6502};

pub struct Implicit;
impl Addressing<Imp> for Implicit {
    fn resolve(_cpu: &mut Cpu6502) -> Imp {
        Imp
    }
}
