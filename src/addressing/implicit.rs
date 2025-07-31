use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::Imp;

pub struct Implicit;
impl Addressing<Imp> for Implicit {
    fn resolve(_cpu: &mut Cpu6502) -> Imp {
        Imp
    }
}
