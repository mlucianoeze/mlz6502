use crate::addressing::absolute::Absolute;
use crate::addressing::immediate::Immediate;
use crate::cpu::Cpu6502;
use crate::instruction::Instruction;
use crate::instruction::isa::isa6502::variant;
use crate::operand::Val;

pub struct Lda;
impl Instruction<Val> for Lda {
    const NAME: &'static str = "lda";

    fn exec(cpu: &mut Cpu6502, Val(v): Val) {
        cpu.a = v;
        cpu.set_zero(cpu.a == 0);
        cpu.set_negative(cpu.a);
    }
}

variant! { 0xA9 => Lda: Immediate(Val) }
variant! { 0xAD => Lda: Absolute(Val) }
