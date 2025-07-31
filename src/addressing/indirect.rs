use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::Addr;

pub struct Indirect;

impl Addressing<Addr> for Indirect {
    fn resolve(cpu: &mut Cpu6502) -> Addr {
        let pointer = cpu.fetch16();
        Addr(cpu.bus.read16(pointer))
    }
}
