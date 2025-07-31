pub mod isa;

pub mod cpx;
pub mod jam;
pub mod lda;
pub mod sta;

use core::marker::PhantomData;
use crate::addressing::Addressing;
use crate::cpu::Cpu6502;
use crate::operand::Operand;

pub trait Instruction<O: Operand> {
    const NAME: &'static str;
    const ILLEGAL: bool = false;
    fn exec(cpu: &mut Cpu6502, op: O);
}

#[derive(Copy, Clone)]
pub struct InstructionEntry {
    pub name: &'static str,
    pub opcode: u8,
    pub illegal: bool,
    pub cycles: u8,
    pub handler: fn(&mut Cpu6502),
}

impl InstructionEntry {
    pub const fn new<I: Instruction<O>, O: Operand>(opcode: u8, handler: fn(&mut Cpu6502)) -> Self {
        Self {
            name: I::NAME,
            opcode,
            illegal: I::ILLEGAL,
            cycles: 0, // TODO: get cycles from instruction
            handler,
        }
    }
}

pub struct InstructionVariant<I, AM, O>(PhantomData<(I, AM, O)>)
where
    O: Operand,
    I: Instruction<O>,
    AM: Addressing<O>;

impl<I, AM, O> InstructionVariant<I, AM, O>
where
    O: Operand,
    I: Instruction<O>,
    AM: Addressing<O>,
{
    fn exec(cpu: &mut Cpu6502) {
        let op = AM::resolve(cpu);
        I::exec(cpu, op);
    }

    pub const fn entry(opcode: u8) -> InstructionEntry {
        InstructionEntry::new::<I, O>(opcode, Self::exec)
    }
}
