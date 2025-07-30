use crate::read_little_endian;

pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);

    fn read16(&self, addr: u16) -> u16 {
        read_little_endian(self.read(addr), self.read(addr.wrapping_add(1)))
    }
}
