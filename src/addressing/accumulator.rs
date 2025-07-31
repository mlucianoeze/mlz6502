use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::Acc;

pub struct Accumulator;
impl Addressing<Acc> for Accumulator {
    fn resolve(_cpu: &mut Cpu6502) -> Acc {
        Acc
    }
}
