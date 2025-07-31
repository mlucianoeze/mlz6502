use crate::addressing::absolute::Absolute;
use crate::addressing::accumulator::Accumulator;
use crate::cpu::Cpu6502;
use crate::instruction::isa::isa6502::variant;
use crate::instruction::Instruction;
use crate::operand::{Acc, Addr};

pub struct Rol;

impl Instruction<Acc> for Rol {
    const NAME: &'static str = "rol";

    fn exec(cpu: &mut Cpu6502, _: Acc) {
        let old_carry = cpu.carry();
        cpu.set_carry(cpu.a & 0x80 != 0);
        cpu.a = (cpu.a << 1) | (old_carry as u8);
        cpu.set_zero(cpu.a == 0);
        cpu.set_negative(cpu.a);
    }
}

impl Instruction<Addr> for Rol {
    const NAME: &'static str = "rol";

    fn exec(cpu: &mut Cpu6502, Addr(addr): Addr) {
        let value = cpu.bus.read(addr);
        let old_carry = cpu.carry();
        cpu.set_carry(value & 0x80 != 0);
        let result = (value << 1) | (old_carry as u8);
        cpu.bus.write(addr, result);
        cpu.set_zero(result == 0);
        cpu.set_negative(result);
    }
}

variant! { 0x2A => Rol: Accumulator(Acc) }
variant! { 0x2E => Rol: Absolute(Addr) }
