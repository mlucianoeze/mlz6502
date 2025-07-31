use crate::addressing::absolute::Absolute;
use crate::cpu::Cpu6502;
use crate::instruction::isa::isa6502::variant;
use crate::instruction::Instruction;
use crate::operand::Addr;

pub struct Sta;
impl Instruction<Addr> for Sta {
    const NAME: &'static str = "sta";

    fn exec(cpu: &mut Cpu6502, Addr(addr): Addr) {
        cpu.bus.write(addr, cpu.a);
    }
}

variant! { 0x8D => Sta: Absolute(Addr) }
