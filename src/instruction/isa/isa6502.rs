use std::sync::LazyLock;
use linkme::distributed_slice;
use crate::addressing::implicit::Implicit;
use crate::instruction::{InstructionEntry, InstructionVariant};
use crate::instruction::jam::Jam;
use crate::operand::Imp;

/// This macro registers each variant in the instruction set.
///
/// # Examples
///
/// ```
/// use mlz6502::instruction::isa::isa6502::variant;
///
/// variant!{ 0xA9 => Lda: Immediate(Val) }
/// ```
macro_rules! variant {
    ($opcode:expr => $instruction:ident: $addressing_mode:ident($operand:ident)) => {
        ::paste::paste! {
            #[linkme::distributed_slice($crate::instruction::isa::isa6502::INSTRUCTIONS_6502_REGISTRY)]
            static [<__REG_6502_ $opcode:upper _ $instruction:upper _ $addressing_mode:upper _ $operand:upper>]: $crate::instruction::InstructionEntry = $crate::instruction::InstructionVariant::<$instruction, $addressing_mode, $operand>::entry($opcode);
        }
    };
}
pub(crate) use variant;

/// This instruction set is populated by the [`variant!`] macro.
pub static INSTRUCTIONS_6502: LazyLock<[InstructionEntry; 256]> = LazyLock::new(|| {
    let mut instructions = [InstructionVariant::<Jam, Implicit, Imp>::entry(0x92); 256];
    for e in INSTRUCTIONS_6502_REGISTRY {
        instructions[e.opcode as usize] = *e;
    }
    instructions
});

#[distributed_slice]
pub static INSTRUCTIONS_6502_REGISTRY: [InstructionEntry] = [..];
