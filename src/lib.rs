pub mod addressing;
pub mod bus;
pub mod cpu;
pub mod instruction;
pub mod memory;
pub mod operand;

#[inline]
fn read_little_endian(lo: u8, hi: u8) -> u16 {
    (hi as u16) << 8 | lo as u16
}
