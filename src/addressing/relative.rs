use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::Rel;

pub struct Relative;
impl Addressing<Rel> for Relative {
    fn resolve(_cpu: &mut Cpu6502) -> Rel {
        todo!()
    }
}
