use crate::addressing::absolute::Absolute;
use crate::addressing::accumulator::Accumulator;
use crate::cpu::Cpu6502;
use crate::instruction::isa::isa6502::variant;
use crate::instruction::Instruction;
use crate::operand::{Acc, Addr};

pub struct Lsr;

impl Instruction<Acc> for Lsr {
    const NAME: &'static str = "lsr";

    fn exec(cpu: &mut Cpu6502, _: Acc) {
        cpu.set_carry(cpu.a & 0x01 != 0);
        cpu.a = cpu.a >> 1;
        cpu.set_zero(cpu.a == 0);
        cpu.set_negative(cpu.a);
    }
}

impl Instruction<Addr> for Lsr {
    const NAME: &'static str = "lsr";

    fn exec(cpu: &mut Cpu6502, Addr(addr): Addr) {
        let value = cpu.bus.read(addr);
        cpu.set_carry(value & 0x01 != 0);
        let result = value >> 1;
        cpu.bus.write(addr, result);
        cpu.set_zero(result == 0);
        cpu.set_negative(result);
    }
}

variant! { 0x4A => Lsr: Accumulator(Acc) }
variant! { 0x4E => Lsr: Absolute(Addr) }
