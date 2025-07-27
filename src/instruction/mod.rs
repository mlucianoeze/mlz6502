pub mod cpx;
pub mod jam;
pub mod lda;
pub mod sta;

use crate::{addressing::Addressing, operand::Operand, Bus, Cpu6502};
use core::marker::PhantomData;

pub trait Instruction<B: Bus, O: Operand> {
    fn exec(cpu: &mut Cpu6502<B>, op: O);
    fn illegal() -> bool {
        false
    }

    fn name<I: Instruction<B, O>>() -> &'static str {
        std::any::type_name::<I>()
            .split("::")
            .last()
            .unwrap_or("???")
    }
}

#[derive(Copy, Clone)]
pub struct InstructionEntry<B: Bus> {
    name: &'static str,
    pub illegal: bool,
    pub cycles: u8,
    pub handler: fn(&mut Cpu6502<B>),
}

impl<B: Bus> InstructionEntry<B> {
    pub fn new<I: Instruction<B, O>, O: Operand>(handler: fn(&mut Cpu6502<B>)) -> Self {
        Self {
            name: I::name::<I>(),
            illegal: I::illegal(),
            cycles: 0, // TODO: get cycles from instruction
            handler,
        }
    }

    pub fn name(&self) -> String {
        self.name.to_lowercase()
    }
}

pub struct InstructionVariant<B: Bus, I, AM, O>(PhantomData<(B, I, AM, O)>)
where
    O: Operand,
    I: Instruction<B, O>,
    AM: Addressing<B, O>;

impl<B: Bus, I, AM, O> InstructionVariant<B, I, AM, O>
where
    O: Operand,
    I: Instruction<B, O>,
    AM: Addressing<B, O>,
{
    fn exec(cpu: &mut Cpu6502<B>) {
        let op = AM::resolve(cpu);
        I::exec(cpu, op);
    }

    pub fn entry() -> InstructionEntry<B> {
        InstructionEntry::new::<I, O>(Self::exec)
    }
}
