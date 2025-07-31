use crate::addressing::absolute::Absolute;
use crate::addressing::accumulator::Accumulator;
use crate::cpu::Cpu6502;
use crate::instruction::isa::isa6502::variant;
use crate::instruction::Instruction;
use crate::operand::{Acc, Addr};

pub struct Ror;

impl Instruction<Acc> for Ror {
    const NAME: &'static str = "ror";

    fn exec(cpu: &mut Cpu6502, _: Acc) {
        let old_carry = cpu.carry();
        cpu.set_carry(cpu.a & 0x01 != 0);
        cpu.a = (cpu.a >> 1) | ((old_carry as u8) << 7);
        cpu.set_zero(cpu.a == 0);
        cpu.set_negative(cpu.a);
    }
}

impl Instruction<Addr> for Ror {
    const NAME: &'static str = "ror";

    fn exec(cpu: &mut Cpu6502, Addr(addr): Addr) {
        let value = cpu.bus.read(addr);
        let old_carry = cpu.carry();
        cpu.set_carry(value & 0x01 != 0);
        let result = (value >> 1) | ((old_carry as u8) << 7);
        cpu.bus.write(addr, result);
        cpu.set_zero(result == 0);
        cpu.set_negative(result);
    }
}

variant! { 0x6A => Ror: Accumulator(Acc) }
variant! { 0x6E => Ror: Absolute(Addr) }
